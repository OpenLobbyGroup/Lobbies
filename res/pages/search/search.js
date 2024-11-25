import { CookieLogin } from "../util/helper.js";
import { Post } from "../util/helper.js";

// only show user button if not registered
const loggedin = await CookieLogin();
showUserBtn();
async function showUserBtn() {
    const btn = document.getElementById("user");
    btn.style.display = !loggedin ? 'inline' : 'none'; // display icon if not logged in
    btn.onclick = () => window.location = "../signup";
}

// inital list
let query = JSON.stringify({
    name: "hott",
});
List(query);
document.getElementById("query").addEventListener('click', () => {
    let query = JSON.stringify({
        name: document.getElementById("search").value,
    });
    List(query);
});
async function List(query) {
    let res = await Post("query", query);
    if (res.ok) {

        let lobbies = await res.json();
        let entries = Object.entries(lobbies);
        document.getElementById("list").innerHTML = '';
        
        for (let i = 0; i < entries.length; i++) {
            let lobby = await Post("lobbies", "", `${entries[i][1]}`);
            if (lobby.ok) {
                let div = createCard((await lobby.json()).game);
                document.getElementById("list").appendChild(div);
            }
        }
    }
    else {
        console.log(await res.text());
    }
}
function createCard(lobby) {
    let div = document.createElement("div");
    let h2 = document.createElement("h2");
    let p = document.createElement("p");

    let card_toolbar = document.createElement("div");
    let like = document.createElement("button");
    let view = document.createElement("button");
    like.textContent = '➕';
    view.textContent = '👁';
    card_toolbar.appendChild(like);
    card_toolbar.appendChild(view);

    div.className = "card";
    card_toolbar.className = "card-toolbar";
    div.id = lobby.name;
    h2.textContent = lobby.name;
    p.textContent = lobby.long_desc.length == 0 ? lobby.short_desc : lobby.long_desc;

    div.appendChild(h2);
    div.appendChild(p);
    div.appendChild(card_toolbar);
    view.addEventListener('click', () => {
        localStorage.setItem("display_lobby", `${name}`)
        return window.location = `../lobby`;
    });

    return div;
}
