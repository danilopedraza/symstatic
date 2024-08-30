const editor = ace.edit("editor");
editor.setTheme("ace/theme/dracula");
editor.session.setMode("ace/mode/javascript");

const results = ace.edit("results");
results.setTheme("ace/theme/dracula");
results.setReadOnly(true);
results.setShowPrintMargin(false);
results.renderer.setShowGutter(false);
results.setHighlightActiveLine(false);
results.renderer.$cursorLayer.element.style.display = "none";

import init, { run_code } from "./komodo/komodo_browser.js"
await init();

let run = () => {
  let res = run_code(editor.getValue(), "");
  results.setValue(res);
  results.clearSelection();
};

document.addEventListener("keydown", (e) => {
  if (e.key === "Enter" && e.ctrlKey) run();
});

document
  .getElementById("exec-button")
  .addEventListener("click", run);
