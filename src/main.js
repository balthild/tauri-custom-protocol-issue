const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // windows
  const base = 'https://myproto.localhost';
  // linux
  // const base = 'myproto://localhost';
  greetMsgEl.src = `${base}/greet/${greetInputEl.value}`;
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});
