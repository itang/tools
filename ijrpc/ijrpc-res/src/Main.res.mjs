// Generated by ReScript, PLEASE EDIT WITH CARE

import * as FunSdk from "./lib/FunSdk.res.mjs";
import * as Core__Option from "@rescript/core/src/Core__Option.res.mjs";
import * as Nodeprocess from "node:process";

async function run(args) {
  let url = Core__Option.getOr(args[0], "http://127.0.0.1:8080/api");
  console.log(">>get", "url       =", url);
  let sdk = FunSdk.make(url);
  let ret = await FunSdk.Ops.getServerInfo(sdk.ops);
  console.log();
  console.log("<<response", "data =", JSON.stringify(ret, undefined, 2));
}

async function main() {
  let args = Nodeprocess.argv.slice(2);
  return await run(args);
}

await main();

/*  Not a pure module */