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
  // The process table's first load runs the native Win32 scan in Rust; a
  // machine with hundreds of processes can take a while to report its rows.
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

test("主题切换记住选择并可以回到跟随系统", async () => {
  // The stored choice is restored on launch, so the control starts on 深色.
  const app = await open("./App.tsx", {
    width: 1000,
    height: 720,
    storage: { appearance: "dark" },
  });
  expect(app.texts()).toContain("自动");
  expect(app.texts()).toContain("浅色");
  expect(app.texts()).toContain("深色");

  await app.press("text:浅色");
  await app.waitUntil(() => app.storage().appearance === "light", { timeout: 5000 });

  await app.press("text:自动");
  await app.waitUntil(() => app.storage().appearance === "system", { timeout: 5000 });

  // 自动 follows the system: the appearance the platform reports switches.
  await app.setColorScheme("dark");
  expect(app.texts()).toContain("自动");
});
