import { invoke } from "@tauri-apps/api/tauri";

let listDiv: HTMLElement | null;

window.addEventListener("DOMContentLoaded", async () => {
    listDiv = document.querySelector("#list-div");
    let list = await post_list();
    list.forEach((sublist) => {
        const div = document.createElement("div");
        div.classList.add("post-div");
        sublist.forEach((item, index) => {
            const p = document.createElement("p");
            switch (index) {
                case 0:
                    p.classList.add("post-id");
                    break;
                case 1:
                    p.classList.add("post-title");
                    break;
                case 2:
                    p.classList.add("post-date");
                    break;
                default:
                    break;
            }
            p.textContent = String(item);
            div.appendChild(p);
        });
        let deleteButton = document.createElement("button");
        deleteButton.classList.add("del-button")
        deleteButton.textContent = "Delete";
        div.appendChild(deleteButton); // Append the delete button to the div

        // Add event listener to the delete button
        deleteButton.addEventListener("click", () => {
            delete_post(sublist[0])
            div.remove()
        });
        listDiv?.appendChild(div)
    });
});

async function post_list(): Promise<(string)[][]> {
    return await invoke("post_list");
}

async function delete_post(id: string) {
    await invoke("delete_post", {
        id: id
    });
}