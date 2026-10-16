import { describe, expect, test } from "bun:test";
import { TaskQueue } from "../src/queue.js";

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

describe("TaskQueue", () => {
  test("caps concurrency and returns results per request", async () => {
    const queue = new TaskQueue(2);
    let active = 0;
    let maxActive = 0;

    const run = (id, ms) =>
      queue.add(async () => {
        active += 1;
        maxActive = Math.max(maxActive, active);
        await sleep(ms);
        active -= 1;
        return id;
      });

    const results = await Promise.all([run(1, 30), run(2, 10), run(3, 5), run(4, 5)]);

    expect(maxActive).toBeLessThanOrEqual(2);
    expect(results).toEqual([1, 2, 3, 4]);
  });

  test("reports queued plus running size", async () => {
    const queue = new TaskQueue(1);
    const first = queue.add(() => sleep(20));
    const second = queue.add(() => sleep(1));
    expect(queue.size).toBe(2);
    await Promise.all([first, second]);
    expect(queue.size).toBe(0);
  });
});
