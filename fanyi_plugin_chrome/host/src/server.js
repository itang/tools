/** Request dispatcher: turns protocol requests into response frames. */

import { ERROR_CODES, MAX_FRAME_BYTES } from "./protocol.js";
import { HostError } from "./errors.js";
import { chunkItems } from "./framing.js";

/** @param {any} value */
function normalizeSegments(value) {
  if (!Array.isArray(value)) return [];
  const segments = [];
  for (const item of value) {
    if (!item || typeof item.text !== "string" || item.text.length === 0) continue;
    const index = Number.isInteger(item.index) ? item.index : segments.length;
    segments.push({ index, text: item.text });
  }
  return segments;
}

/**
 * @param {object} deps
 * @param {import("./engine.js").TranslationEngine} deps.engine
 * @param {() => Promise<any>} deps.health
 * @param {(message: unknown) => void} deps.emit  Writes one outbound frame.
 */
export function createDispatcher({ engine, health, emit }) {
  /**
   * @param {any} request
   * @returns {Promise<void>}
   */
  return async function dispatch(request) {
    const id = typeof request?.id === "string" ? request.id : "";
    let seq = 0;

    if (!request || typeof request !== "object") {
      emit({ id, seq: seq++, last: true, type: "error", error: { code: ERROR_CODES.BAD_REQUEST, message: "请求格式不正确" } });
      return;
    }

    if (request.type === "health") {
      const data = await health();
      emit({ id, seq: seq++, last: true, type: "health", data });
      return;
    }

    if (request.type === "translate") {
      const segments = normalizeSegments(request.segments);
      if (segments.length === 0) {
        emit({ id, seq: seq++, last: true, type: "translate", translations: [] });
        return;
      }
      try {
        const translations = await engine.translate(segments, request.options || {}, (progress) => {
          emit({ id, seq: seq++, last: false, type: "progress", ...progress });
        });

        const render = (chunk) => ({ id, seq: 0, last: false, type: "translate", translations: chunk });
        const chunks = chunkItems(translations, render, MAX_FRAME_BYTES);
        chunks.forEach((chunk, i) => {
          emit({ id, seq: seq++, last: i === chunks.length - 1, type: "translate", translations: chunk });
        });
      } catch (error) {
        const code = error instanceof HostError ? error.code : ERROR_CODES.HOST_ERROR;
        emit({
          id,
          seq: seq++,
          last: true,
          type: "error",
          error: { code, message: String(error?.message ?? error) },
        });
      }
      return;
    }

    emit({
      id,
      seq: seq++,
      last: true,
      type: "error",
      error: { code: ERROR_CODES.BAD_REQUEST, message: `未知请求类型：${request.type}` },
    });
  };
}
