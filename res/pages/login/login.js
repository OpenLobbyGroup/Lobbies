import { ValidatePassword } from "../util/helper.js";
import { ValidateUsername } from "../util/helper.js";
import { Post } from "../util/helper.js";
import { CookieLogin } from "../util/helper.js"

if (await CookieLogin())
    window.location = "../search";

async function login(username, password, output) {
    output.value = "";
    try {
        ValidateUsername(user.value);
    }
    catch (e) {
        output.value = "Unable to validate the username";

    }
    try {
        ValidatePassword(password.value);
    }
    catch (e) {
        output.value += "Unable to validate the password";
    }
    if (output.value != "")
        return;

    const data = JSON.stringify({
        name: user.value,
        password: password.value
    });
    const remember = document.getElementById("remember").checked ? "true" : "false";

    const response = await Post("login", data, remember);
    if (response.ok) {
        output.value = "Login Successful, setting up the session!";
        const sesh = await Post("sesh", data);
        if (sesh.ok) {
            output.value = "Sesion is ready, redirecting soon!";
            setTimeout(() => window.location = "../search", 2000);
        }
        else {
            output.value = "Unable to setup the session!";
            console.log(await sesh.text());
        }
    }
    else
        output.value = await response.text();
}

window.login = login;