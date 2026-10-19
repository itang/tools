import { describe, expect, test } from "bun:test";
import { md5 } from "../src/shared/md5.js";
import { genSign, normalizeLang } from "../src/shared/youdao.js";
import { DEFAULT_SETTINGS, normalizeSettings } from "../src/shared/settings.js";

describe("md5", () => {
  test("matches RFC 1321 vectors", () => {
    expect(md5("")).toBe("d41d8cd98f00b204e9800998ecf8427e");
    expect(md5("abc")).toBe("900150983cd24fb0d6963f7d28e17f72");
    expect(md5("message digest")).toBe("f96b697d7cb7938d525a2f31aaf161d0");
    expect(md5("hello world")).toBe("5eb63bbbe01eeed093cb22bb8f5acdc3");
    expect(md5("中文测试")).toBe("089b4943ea034acfa445d050c7913e55");
  });
});

describe("youdao genSign", () => {
  test("reproduces a captured webtranslate/sse signature", () => {
    const params = {
      product: "webfanyi",
      appVersion: "1",
      client: "webmain",
      mid: "1",
      vendor: "web",
      screen: "1",
      model: "1",
      imei: "1",
      network: "wifi",
      keyfrom: "webfanyi.webmain",
      keyid: "translate-webfanyi-webmain",
      mysticTime: "1792380558371",
      yduuid: "2bb859491a3c831be8044678cf69460a",
      modelName: "llmLite",
      useTerm: "false",
      i: "Hello%20world%2C%20this%20is%20a%20test%20for%20translation.%20second%20probe%20284%20xyz7041",
      from: "en",
      to: "zh-CHS",
      signSecretKey: "BdCYRtHAJxO7yNm9RHwU2JiFISIk62Ts",
      keyId: "translate-webfanyi-webmain",
      token: "5e82f886917643fcb19dd01a7a50d9bd",
      source: "webmain",
    };
    const { sign, pointParam } = genSign(params, "BdCYRtHAJxO7yNm9RHwU2JiFISIk62Ts");
    expect(sign).toBe("aec31d67e879fee4f3d904a0440c30ec");
    expect(pointParam).toBe(
      "appVersion,client,from,i,imei,keyId,keyfrom,keyid,mid,model,modelName,mysticTime,network,product,screen,signSecretKey,source,to,token,useTerm,vendor,yduuid,key",
    );
  });

  test("drops empty values before signing", () => {
    const a = genSign({ b: "2", e: "" }, "sk");
    const b = genSign({ b: "2" }, "sk");
    expect(a.sign).toBe(b.sign);
    expect(a.pointParam).toBe("b,key");
  });
});

describe("youdao normalizeLang", () => {
  test("maps common targets to youdao codes", () => {
    expect(normalizeLang("zh-CN")).toBe("zh-CHS");
    expect(normalizeLang("zh-TW")).toBe("zh-CHT");
    expect(normalizeLang("ja-JP")).toBe("ja");
    expect(normalizeLang("en")).toBe("en");
    expect(normalizeLang("")).toBe("zh-CHS");
  });

  test("passes unknown codes through", () => {
    expect(normalizeLang("sw")).toBe("sw");
    expect(normalizeLang("fr-FR")).toBe("fr");
  });
});

describe("backend setting", () => {
  test("defaults to opencode", () => {
    expect(DEFAULT_SETTINGS.backend).toBe("opencode");
    expect(normalizeSettings({}).backend).toBe("opencode");
  });

  test("accepts youdao and rejects unknown values", () => {
    expect(normalizeSettings({ backend: "youdao" }).backend).toBe("youdao");
    expect(normalizeSettings({ backend: "google" }).backend).toBe("opencode");
  });
});
