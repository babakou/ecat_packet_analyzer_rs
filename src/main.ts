import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let greetInput2El : HTMLInputElement | null;
let greetMsg2El : HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // greetMsgEl.textContent = await invoke("greet", {
    //   name: greetInputEl.value,
    // });
    await invoke("greet_async", {
      name: greetInputEl.value,
    });
  }
}

async function greet2() {
  if (greetMsg2El && greetInput2El) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg2El.textContent = await invoke("greet2", {
      name: greetInput2El.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  greetInput2El = document.querySelector("#greet-input2");
  greetMsg2El = document.querySelector("#greet-msg2");
  document.querySelector("#greet-form2")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet2();
  });
});

listen<string>('greet', (greet) => {
  if (greetMsgEl) {
    greetMsgEl.textContent = greet.payload;
  }
})
