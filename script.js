import init, * as wasm from "./pkg/brainfuck_wasm.js";

const code = document.getElementById('code');
const runButton = document.getElementById("runButton");
const output = document.getElementById("output");

const buttons = document.querySelectorAll("button");
let interpreter;

init().then(() => {
    main();
});

function main() {
    runButton?.addEventListener("click", onButtonClick);
    buttons.forEach(button => {
        if (button.parentElement.id != "keys") {
            return
        }

        button.addEventListener("click", () => {
            if (button.innerText == "↩️") {
                code.innerHTML += "\n";
            } else {
                code.innerHTML += button.innerText;
            }
        });
    });
}

function onButtonClick() {
    output.innerHTML = "";
    interpreter = wasm.new_interpreter(code.value);
    interpreter.run();
}