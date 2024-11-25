// tabs
const lobby_selector = document.getElementById("lobby-selector");
const server_selector = document.getElementById("server-selector");

const lobby_tab = document.getElementById("lobby-tab");
const server_tab = document.getElementById("server-tab");

let current_selector = lobby_selector;
let current_tab = lobby_tab;
lobby_tab.style.display = 'none';
server_tab.style.display = 'none';

lobby_selector.onclick = () => changeTab("lobby");
server_selector.onclick = () => changeTab("server");

changeTab("lobby");
function changeTab(tab) {
    switch (tab) {
        case "lobby":
            updateTab(lobby_selector, lobby_tab);
            break;
        case "server":
            updateTab(server_selector, server_tab);
            break;
        default:
            break;
    }

    function updateTab(selector, tab) {
        current_tab.style.display = 'none';
        current_selector.classList.remove('checked');

        current_selector = selector;
        current_tab = tab;

        current_selector.classList.add('checked');
        current_tab.style.display = 'flex';
    }
}

// submit
import { ValidateIP } from "../../util/helper.js";
import { Post } from "../util/helper.js";
import { ValidatePassword } from "../util/helper.js";
const btn = document.getElementById("submit");
const op = document.getElementById("submit-output");
btn.onclick = Submit;

async function Submit() {
    let brand = document.getElementById("game-name");
    let name = document.getElementById("lobby-name");
    let short_desc = document.getElementById("short-desc");
    let long_desc = document.getElementById("long-desc");
    let address = document.getElementById("address");
    let port = document.getElementById("port");
    let password = document.getElementById("password");

    let error = false;
    InRange(brand, 4, 9);
    InRange(name, 4, 9);
    InRange(short_desc, 40, 120);
    InRange(long_desc, 0, 1000);
    function InRange(str, min, max) {
        if (str.value.length < min || str.value.length > max) {
            str.nextElementSibling.value = `Must be between ${min}-${max} characters`;
            error = true;
        } else {
            str.nextElementSibling.value = "";
        }
    }

    try {
        address.nextElementSibling.value = "";
        ValidateIP(address.value);
    }
    catch (e) {
        address.nextElementSibling.value = e;
        error = true;
    }
    if (port.value < 1 || port.value > 65536) {
        port.nextElementSibling.value = "Must be between 1-65536";
        error = true;
    } else {
        port.nextElementSibling.value = "";
    }
    try {
        password.nextElementSibling.value = "";
        ValidatePassword(password.value);
    }
    catch (e) {
        password.nextElementSibling.value = e;
        error = true;
    }

    if (error) {
        op.value = "There was a validation error";
        return;
    }

    const lobby = JSON.stringify(
        {
            game: {
                brand: brand.value,
                name: name.value,
                short_desc: short_desc.value,
                long_desc: long_desc.value,
            },
            settings: {
                address: address.value,
                port: Number(port.value),
                password: password.value,
            }

        }
    );

    let res = await Post("create", lobby);
    if (res.ok)
    {
        let msg = await res.text();
        op.value = msg;
        console.log(msg);

        window.localStorage.setItem("display_lobby", `${name.value}`)
        window.location = `../lobby`;
    }
    else if (res.status == 401) // unauthorized, it was caught by the server
    {
        let msg = await res.text();
        op.value = msg;
        console.log(msg);
    }
    else // some other error
    {
        let msg = await res.text();
        op.value = "There was an error";
        console.log(msg);
    }
}