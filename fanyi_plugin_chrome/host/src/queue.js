/** Bounded-concurrency FIFO task queue. */

export class TaskQueue {
  /** @param {number} [concurrency] */
  constructor(concurrency = 2) {
    this.concurrency = Math.max(1, Number(concurrency) || 1);
    this.active = 0;
    /** @type {{task: () => Promise<any>, resolve: (v:any)=>void, reject:(e:any)=>void}[]} */
    this.pending = [];
  }

  /** Total queued plus running tasks. */
  get size() {
    return this.pending.length + this.active;
  }

  /**
   * Enqueue a task. Tasks start in FIFO order, at most `concurrency` at once.
   * @template T
   * @param {() => Promise<T>} task
   * @returns {Promise<T>}
   */
  add(task) {
    return new Promise((resolve, reject) => {
      this.pending.push({ task, resolve, reject });
      this.#drain();
    });
  }

  #drain() {
    while (this.active < this.concurrency && this.pending.length > 0) {
      const entry = this.pending.shift();
      this.active += 1;
      Promise.resolve()
        .then(entry.task)
        .then(entry.resolve, entry.reject)
        .finally(() => {
          this.active -= 1;
          this.#drain();
        });
    }
  }
}
