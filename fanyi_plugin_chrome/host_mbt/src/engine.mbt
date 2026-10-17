///| Translation engine: cache lookup, batching, queueing, error mapping.

///|
/// Ties cache + queue + batching together.
pub struct TranslationEngine {
  runner : &Runner
  queue : @semaphore.Semaphore
  cache : LruCache
  timeout_ms : Int32
  default_model : String
}

///|
/// Create an engine around a runner, with bounded concurrency and an LRU
/// cache.
pub fn TranslationEngine::new(
  runner : &Runner,
  concurrency~ : Int32 = 2,
  cache_max~ : Int32 = 2000,
  timeout_ms~ : Int32 = 120_000,
  default_model~ : String = "",
) -> TranslationEngine {
  {
    runner,
    queue: @semaphore.Semaphore(if concurrency < 1 { 1 } else { concurrency }),
    cache: LruCache::new(cache_max),
    timeout_ms,
    default_model,
  }
}

///|
/// Run one model call for a batch, mapping failures to `HostError`.
async fn TranslationEngine::call_model_once(
  self : TranslationEngine,
  batch : Array[TranslateSegment],
  target_lang : String,
  model : String,
  agent : String?,
  timeout_ms : Int32?,
) -> String {
  let result = self.runner.run_once(
    build_instruction(target_lang),
    build_stdin_payload(batch),
    if model.length() > 0 {
      Some(model)
    } else {
      None
    },
    agent,
    match timeout_ms {
      Some(value) => Some(value)
      None => Some(self.timeout_ms)
    },
  )
  match map_opencode_failure(result) {
    Some(error) => raise error
    None => result.text
  }
}

///|
/// Translate segments, serving repeats from cache, batching the misses, and
/// reporting progress through `on_progress`.
pub async fn TranslationEngine::translate(
  self : TranslationEngine,
  segments : Array[TranslateSegment],
  options : TranslateOptions,
  on_progress : async (Progress) -> Unit,
) -> Array[TranslationResult] {
  let target_lang = if options.target_lang.length() > 0 {
    options.target_lang
  } else {
    "zh-CN"
  }
  let model = if options.model.length() > 0 {
    options.model
  } else {
    self.default_model
  }
  let total = segments.length()
  let results : Array[TranslationResult?] = Array::make(total, None)
  // Group positions by cache key so identical text is translated once.
  let key_to_positions : Map[String, Array[Int32]] = Map([])
  let unique : Array[(String, TranslateSegment)] = []
  for position = 0; position < total; position = position + 1 {
    let segment = segments[position]
    let key = cache_key(target_lang, model, segment.text)
    match key_to_positions.get(key) {
      Some(positions) => positions.push(position)
      None => {
        key_to_positions.set(key, [position])
        unique.push((key, segment))
      }
    }
  }
  let misses : Array[(String, TranslateSegment)] = []
  let mut completed = 0
  for entry in unique {
    let (key, segment) = entry
    let positions = key_to_positions.get(key).unwrap()
    match self.cache.get(key) {
      Some(cached) => {
        for position in positions {
          results[position] = Some(
            TranslationResult::ok(segments[position].index, cached),
          )
        }
        completed = completed + positions.length()
      }
      None => misses.push((key, segment))
    }
  }
  on_progress({ phase: "cache", completed, total, })
  if misses.length() > 0 {
    let mut done = completed
    let call_model : async (Array[TranslateSegment]) -> String = batch => {
      self.queue.acquire()
      defer self.queue.release()
      let raw = self.call_model_once(
        batch,
        target_lang,
        model,
        options.agent,
        options.timeout_ms,
      )
      done = done + batch.length()
      let reported = if done > total { total } else { done }
      on_progress({ phase: "batch", completed: reported, total, })
      raw
    }
    let miss_segments = misses.map(entry => entry.1)
    let batch_results = translate_batch(miss_segments, call_model)
    let by_index : Map[Int32, TranslationResult] = Map([])
    for result in batch_results {
      by_index.set(result.index, result)
    }
    for entry in misses {
      let (key, segment) = entry
      let positions = key_to_positions.get(key).unwrap()
      let outcome : TranslationResult? = match by_index.get(segment.index) {
        Some(result) =>
          match result.text {
            Some(text) => {
              self.cache.set(key, text)
              Some(TranslationResult::ok(segment.index, text))
            }
            None => None
          }
        None => None
      }
      for position in positions {
        results[position] = match outcome {
          Some(result) =>
            Some(
              TranslationResult::ok(
                segments[position].index,
                result.text.unwrap(),
              ),
            )
          None =>
            Some(
              TranslationResult::failed(segments[position].index, Incomplete),
            )
        }
      }
    }
  }
  on_progress({ phase: "done", completed: total, total, })
  let out : Array[TranslationResult] = []
  for position = 0; position < total; position = position + 1 {
    out.push(
      match results[position] {
        Some(result) => result
        None => TranslationResult::failed(segments[position].index, Incomplete)
      },
    )
  }
  out
}
