import { invoke } from "@tauri-apps/api/core";

let bytesInputEl : HTMLInputElement | null;
let analyzeResultTableEl : HTMLTableElement | null | undefined;

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
  let analyzeResultIFrameEl: HTMLIFrameElement | null = document.querySelector("#analyze-result-iframe");
  analyzeResultTableEl = analyzeResultIFrameEl?.contentDocument?.body.querySelector("#analyze-result-table");
  bytesInputEl = document.querySelector("#bytes-input");

  document.querySelector("#greet-form2")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet2();
  });

  document.querySelector("#clear-result")?.addEventListener("click", (e) => {
    //e.preventDefault();
    if (analyzeResultTableEl) {
      let rows = analyzeResultTableEl.rows.length;
      if (rows > 1) {
        for (let i = 1; i < rows; i++) {
          e.preventDefault();
          analyzeResultTableEl.deleteRow(1);
        }
      }
    }
  })
});
