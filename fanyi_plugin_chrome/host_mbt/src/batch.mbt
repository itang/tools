///| Batch translation contract: prompt building, response parsing, alignment.

///|
/// Instruction sent to the model. The source segments are supplied on stdin
/// and the model must answer with a JSON object keyed by segment index so that
/// results stay aligned even if the model reorders them.
pub fn build_instruction(target_lang : String) -> String {
  [
    "You are a translation engine.",
    "Translate the \"text\" field of every item in the JSON array provided on stdin into \{target_lang}.",
    "Some texts contain markup tokens: [[n]] and [[/n]] wrap an inline element, {{n}} is an opaque node, {{br}} is a line break.",
    "Keep every token byte-for-byte, in the same order; never translate, drop, renumber, duplicate or add tokens.",
    "Translate only the natural language between tokens.",
    "Preserve proper nouns, @handles, #hashtags, URLs, numbers and code as-is.",
    "Do not translate text that is already in the target language.",
    "Respond with ONLY a JSON object mapping each item index (as a string) to its translation, e.g. {\"0\":\"...\",\"1\":\"...\"}.",
    "No prose, no markdown, no code fences.",
  ].join(" ")
}

///|
/// Serialize segments to the stdin payload sent to the model.
pub fn build_stdin_payload(segments : Array[TranslateSegment]) -> String {
  Json::array(
    segments.map(segment => {
      ({ "index": segment.index, "text": segment.text } : Json)
    }),
  ).stringify()
}

///|
/// Pull a JSON object out of a raw model answer, tolerating markdown fences
/// and surrounding prose.
pub fn parse_translation_response(raw : String) -> Map[String, Json] raise {
  let mut text = raw.trim().to_owned()
  if text.find("```") is Some(start) &&
    text[start + 3:].find("```") is Some(length) {
    let mut inner = text[start + 3:start + 3 + length].trim().to_owned()
    if inner.to_lower().has_prefix("json") {
      inner = inner[4:].trim().to_owned()
    }
    text = inner
  }
  let start = text.find("{")
  let end = text.rev_find("}")
  guard start is Some(start) && end is Some(end) && end >= start else {
    fail("模型未返回 JSON 对象")
  }
  let parsed = @json.parse(text[start:end + 1])
  guard parsed is Object(members) else {
    fail("模型返回的 JSON 不是对象")
  }
  members
}

///|
/// The result of aligning a parsed `index -> translation` mapping back onto
/// the original segments.
pub(all) struct Alignment {
  results : Array[TranslationResult]
  missing : Array[TranslateSegment]
  extra : Array[String]
}

///|
/// Align a parsed `index -> translation` mapping back onto the original
/// segments.
pub fn align_translations(
  segments : Array[TranslateSegment],
  mapping : Map[String, Json],
) -> Alignment {
  let results : Array[TranslationResult] = []
  let missing : Array[TranslateSegment] = []
  let wanted = segments.map(segment => segment.index.to_string())
  for segment in segments {
    let key = segment.index.to_string()
    match mapping.get(key) {
      Some(String(value)) if value.length() > 0 =>
        results.push(TranslationResult::ok(segment.index, value))
      _ => missing.push(segment)
    }
  }
  let extra : Array[String] = []
  for key in mapping.keys() {
    if !wanted.contains(key) {
      extra.push(key)
    }
  }
  { results, missing, extra, }
}

///|
/// Turn a batch response into per-segment results, retrying the segments that
/// the model omitted exactly once.
pub async fn translate_batch(
  segments : Array[TranslateSegment],
  call_model : async (Array[TranslateSegment]) -> String,
) -> Array[TranslationResult] {
  let mapping = parse_translation_response(call_model(segments))
  let aligned = align_translations(segments, mapping)
  if aligned.missing.is_empty() {
    return aligned.results
  }
  let retry_mapping = parse_translation_response(call_model(aligned.missing))
  let retry_aligned = align_translations(aligned.missing, retry_mapping)
  let by_index : Map[Int32, TranslationResult] = Map([])
  for result in aligned.results {
    by_index.set(result.index, result)
  }
  for result in retry_aligned.results {
    by_index.set(result.index, result)
  }
  for segment in retry_aligned.missing {
    by_index.set(
      segment.index,
      TranslationResult::failed(segment.index, Incomplete),
    )
  }
  segments.map(segment => {
    match by_index.get(segment.index) {
      Some(result) => result
      None => TranslationResult::failed(segment.index, Incomplete)
    }
  })
}
