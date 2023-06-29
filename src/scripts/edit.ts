let listDiv: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
    listDiv = document.querySelector("#list-div");
    let list: (number | string)[][] = [
        [1, "test1", "29.06.2023"],
        [2, "test2", "30.06.2023"],
        [3, "test3", "01.07.2023"],
    ]
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
            div.remove()
        });
        listDiv?.appendChild(div)
    });
});