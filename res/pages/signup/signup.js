import { ValidatePassword } from "../util/helper.js";
import { ValidateUsername } from "../util/helper.js";
import { Post } from "../util/helper.js";
import { CookieLogin } from "../util/helper.js";

if (await CookieLogin())
    window.location = "../search";

async function signup(user, password, output) {
    output.value = "";
    try {
        ValidatePassword(password.value);
    }
    catch (e) {
        output.value = e;
    }
    try {
        ValidateUsername(user.value);
    }
    catch (e) {
        output.value += e;
    }
    if (output.value != "")
        return;

    const data = JSON.stringify({
        name: user.value,
        password: password.value
    });
    const remember = document.getElementById("remember").checked ? "true" : "false";
    const response = await Post("signup", data, remember);
    if (response.status == 201) // created
    {
        output.value = "Your have been registered, setting up session now!";
        const sesh = await Post("sesh");
        if (sesh.ok) {
            output.value = "Session ready!";
            setTimeout(() => { output.value = "Redirecting you now!"; window.location = "../search"; }, 2000);
        }
        else
        {
            output.value = "Unable get the session ready!";
        }
    }
    else if (response.status == 200) // already registered
    {
        output.value = "You are already registered, setting up session now!";
        const sesh = await Post("sesh", data);
        if (sesh.ok) {
            output.value = "Session ready!";
            setTimeout(() => { output.value = "Redirecting you now!"; window.location = "../search"; }, 2000);
        }
        else
        {
            output.value = "Unable get the session ready!";
            console.log(await sesh.text());
        }
    }
    else {
        output.value = await response.text();
    }
}

window.signup = signup;