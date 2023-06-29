import { invoke } from "@tauri-apps/api/tauri";

let urlEl: HTMLInputElement | null;
let passEl: HTMLInputElement | null;
let loginButtonEl: HTMLElement | null;
let errorMsgEl: HTMLElement | null;

async function login() { 
    if (urlEl && passEl && loginButtonEl && errorMsgEl) {
        let authenticated = await invoke("login", {
            url: urlEl.value,
            pass: passEl.value
        })
        if (authenticated) {
            window.location.replace('./menu.html')
        }
        else {
            errorMsgEl.textContent = "Wrong URL or password"
        }
    }
}
window.addEventListener("DOMContentLoaded", () => {
    urlEl = document.querySelector("#url")
    passEl = document.querySelector("#password");
    errorMsgEl = document.querySelector("#error-msg");
    loginButtonEl = document.querySelector("#login-button");
    passEl?.addEventListener("keypress", function(event) {
            if (event.key == "Enter") {
                event.preventDefault();
                loginButtonEl?.click();
            }

    });
    document
    .querySelector("#login-button")
    ?.addEventListener("click", () => login());
  });