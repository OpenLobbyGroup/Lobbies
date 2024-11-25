import { Post } from "../util/helper.js"

// set the long desc
const desc = document.getElementById("long-desc");
const lobby_name = localStorage.getItem("display_lobby");
localStorage.removeItem("display_lobby");
if (lobby_name == null) {
    desc.textContent = "Lobby name hasn't been assigned";
    location.href = "../search";
}

const res = await Post(`lobbies/${lobby_name}`);
if (res.ok) {
    desc.textContent = JSON.stringify(await res.json());
}
else {
    desc.textContent = await res.text();
    setTimeout(() => location.href = "../search", 1000);
}
