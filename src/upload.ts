import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog"

let titleEl: HTMLInputElement | null;
let docInputEl: HTMLInputElement | null;
let statusEl: HTMLElement | null;
let imageButtonEl: HTMLElement | null;

async function save_post() {
  if (titleEl && statusEl && imageButtonEl && docInputEl) {
    console.log("asd")
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    statusEl.textContent = await invoke("save_post", {
      title: titleEl.value,
      img_path: imageButtonEl.textContent, 
      content: docInputEl.value

    });
    docInputEl.value = "";
  }
}

async function file_handler() { 
  if (imageButtonEl) {
  let file = await open({filters: [{
    name: 'Image',
    extensions: ['png', 'jpeg']
  }]
  });
  if (typeof file == "string") {
  imageButtonEl.textContent = file
  }}
}
window.addEventListener("DOMContentLoaded", () => {
  titleEl = document.querySelector("#doc-title")
  statusEl = document.querySelector("#status-msg");
  docInputEl = document.querySelector("#doc-text");
  imageButtonEl = document.querySelector("#image-button");
  document.querySelector("#image-button")?.addEventListener("click", () => file_handler());
  document
    .querySelector("#submit-button")
    ?.addEventListener("click", () => save_post());
});
