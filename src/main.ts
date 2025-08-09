import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let bytesInputEl : HTMLInputElement | null;
let analyzeResultTableEl : HTMLTableElement | null;

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

export type DLInfo = {
  addr: number,
  cmd: string,
  pdiwdt: string,
  port0_str: string,
  port1_str: string,
  port2_str: string,
  port3_str: string,
};

async function greet2() {
  if (analyzeResultTableEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const dlinfos: DLInfo[] = await invoke("greet2", {
      name: bytesInputEl?.value,
    });

    dlinfos.forEach(dlinfo => {
      const newTrElement = analyzeResultTableEl?.insertRow();
      const addrTdElement = newTrElement?.insertCell();
      if (addrTdElement) {
        addrTdElement.textContent = dlinfo.addr.toString(10);
      }

      const cmdTdElement = newTrElement?.insertCell();
      if (cmdTdElement) {
        cmdTdElement.textContent = dlinfo.cmd;
      }

      const pdiwdtTdElement = newTrElement?.insertCell();
      if (pdiwdtTdElement) {
        pdiwdtTdElement.textContent = dlinfo.pdiwdt;
      }

      const port0TdElement = newTrElement?.insertCell();
      if (port0TdElement) {
        port0TdElement.textContent = dlinfo.port0_str;
      }
      
      const port1TdElement = newTrElement?.insertCell();
      if (port1TdElement) {
        port1TdElement.textContent = dlinfo.port1_str;
      }

      const port2TdElement = newTrElement?.insertCell();
      if (port2TdElement) {
        port2TdElement.textContent = dlinfo.port2_str;
      }

      const port3TdElement = newTrElement?.insertCell();
      if (port3TdElement) {
        port3TdElement.textContent = dlinfo.port3_str;
      }
    })
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  analyzeResultTableEl = document.querySelector("#analyze-result-table");
  bytesInputEl = document.querySelector("#bytes-input");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

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
