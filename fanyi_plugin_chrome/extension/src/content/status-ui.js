/** Small in-page status pill used for progress and error feedback. */

const UI_ATTR = "data-oc-trans-ui";

export function createStatusUI(doc) {
  let node = null;
  let hideTimer = null;

  function ensure() {
    if (node && node.isConnected) return node;
    node = doc.createElement("div");
    node.setAttribute(UI_ATTR, "");
    node.style.cssText = [
      "position:fixed",
      "right:16px",
      "bottom:16px",
      "z-index:2147483647",
      "max-width:340px",
      "padding:8px 12px",
      "border-radius:8px",
      "background:rgba(17,24,39,.92)",
      "color:#fff",
      "font:12px/1.5 system-ui,-apple-system,Segoe UI,sans-serif",
      "box-shadow:0 4px 16px rgba(0,0,0,.3)",
      "pointer-events:none",
    ].join(";");
    (doc.body || doc.documentElement).appendChild(node);
    return node;
  }

  function show(message, { sticky = false } = {}) {
    const target = ensure();
    target.textContent = message;
    target.style.display = "block";
    clearTimeout(hideTimer);
    if (!sticky) {
      hideTimer = setTimeout(() => {
        target.style.display = "none";
      }, 4000);
    }
  }

  return {
    show,
    progress: (completed, total) => show(`翻译中… ${completed}/${total}`),
    error: (message) => show(message, { sticky: true }),
    hide: () => {
      if (node) node.style.display = "none";
    },
  };
}
