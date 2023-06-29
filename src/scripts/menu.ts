window.addEventListener("DOMContentLoaded", () => {
    document
        .querySelector("#upload-button")
        ?.addEventListener("click", () => window.location.replace("./upload.html"));
});

window.addEventListener("DOMContentLoaded", () => {
    document
        .querySelector("#edit-button")
        ?.addEventListener("click", () => window.location.replace("./edit.html"));
});