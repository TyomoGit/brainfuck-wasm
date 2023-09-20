import init, * as wasm from "./pkg/brainfuck_wasm.js";

const code = document.getElementById('code');
const runButton = document.getElementById("runButton");
const output = document.getElementById("output");

init().then(() => {
    main();
});

function main() {
    runButton?.addEventListener("click", onButtonClick);
}

function onButtonClick() {
    output.innerHTML = "";
    wasm.generate_from_text(code.value);
}