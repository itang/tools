/**
 * Accumulates response frames for one host request and turns the terminal
 * frame into the final result. Kept free of chrome APIs so it is unit-testable.
 */

export function createFrameAccumulator() {
  /** @type {{index:number,text?:string,error?:string}[]} */
  const translations = [];

  return {
    /** Merge a non-terminal frame into the accumulated state. */
    add(frame) {
      if (frame?.type === "translate" && Array.isArray(frame.translations)) {
        translations.push(...frame.translations);
      }
    },

    /** Produce the final result for a terminal frame. */
    resolve(frame) {
      if (frame?.type === "error") {
        return { error: frame.error || { code: "HOST_ERROR", message: "未知错误" } };
      }
      if (frame?.type === "health") {
        return { health: frame.data };
      }
      if (frame?.type === "translate") {
        translations.push(...(frame.translations || []));
        return { translations: [...translations] };
      }
      return { error: { code: "HOST_ERROR", message: `未知响应类型：${frame?.type}` } };
    },
  };
}
