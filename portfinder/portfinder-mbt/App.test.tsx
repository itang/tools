import { expect, open, test } from "nsh/test";

test("opens and reports the outcome of a scan", async () => {
  const app = await open("./App.tsx", { width: 1000, height: 720 });
  expect(app.texts()).toContain("端口雷达");
  expect(app.texts()).toContain("开放端口的进程");
  expect(app.texts()).toContain("查询");
  expect(app.texts()).toContain("清空");
  await app.waitUntil(
    () =>
      app.texts().some(
        (text) =>
          text.includes("查询失败") || text.includes("空闲") || text.includes("正被"),
      ),
    { timeout: 20000 },
  );
});

test("清空 clears both conditions and the port answer", async () => {
  const app = await open("./App.tsx", { width: 1000, height: 720 });
  await app.waitUntil(
    () =>
      app.texts().some(
        (text) => text.includes("查询失败") || text.includes("空闲") || text.includes("正被"),
      ),
    { timeout: 20000 },
  );
  await app.type("label:进程名称", "svc");
  await app.press("text:查询");
  // The process table's first load runs a PowerShell scan; a machine with
  // hundreds of processes takes over ten seconds to report its rows.
  await app.waitUntil(() => app.texts().some((text) => text.includes("名称匹配「svc」")), {
    timeout: 45000,
  });

  await app.press("text:清空");
  await app.waitUntil(
    () =>
      !app
        .texts()
        .some(
          (text) =>
            text.includes("名称匹配") ||
            text.includes("查询失败") ||
            text.includes("空闲") ||
            text.includes("正被"),
        ),
    { timeout: 15000 },
  );
  expect(app.count("text:进程名")).toBeGreaterThan(0);
  expect(app.count("text:端口号")).toBeGreaterThan(0);
});
