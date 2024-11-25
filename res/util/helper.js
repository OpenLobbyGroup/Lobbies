// export a fun to post with fetch
export async function Post(api, json = "", parms = "") {
    return await fetch(
        `https://localhost/${api}` + (parms == "" ? "" : "/" + parms),
        {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: json
        },
    );
}
export async function CookieLogin() {
    try {
        const r = await Post("profile");
        console.log(await r.text());
        if (r.ok) {
            return true;
        }
    }
    catch {
        console.log("Unable to login using cookies");
    }
    return false;
}

export function ValidatePassword(pw) {
    let msg = [];
    // 8-16 chars
    if (pw.length < 8 || pw.length > 16)
        msg.push("Password must contain 8-16 characters");

    // Requirements
    if (/[A-Z]/.test(pw) == false) msg.push(" Capital letter");
    if (/[a-z]/.test(pw) == false) msg.push(" Lowercase letter");
    if (/[0-9]/.test(pw) == false) msg.push(" Number");
    if (/[!@#$%^&*()_+\-={};':"|,.<>?]/.test(pw) == false) msg.push(" Symbol");

    if (msg.length != 0)
        throw msg;
    else
        return 'Password is valid';
}

export function ValidateUsername(username) {
    let msg = [];
    if (username.length < 5 || username.length > 9) {
        msg.push("Username must contain 5-9 characters in length");
        msg.push("Username may only contain letters, numbers, and underscores");
    }

    if (/[^A-Za-z0-9_]/.test(username))
        msg.push("Username may only contain letters, numbers, and underscores");

    if (msg.length != 0)
        throw msg;
    else
        return "Username is valid";
}

export function ValidateIP(ip) {
    ip = ip.trim();
    if (ip.length == 0) throw new Error("IP required");

    var parts = ip.split(".");
    if (parts.length < 4) throw new Error("IP doesn't have enough parts");
    if (parts.length == 4 && parts[3].length == 0) throw new Error("All 4 parts must be entered");
    if (parts.length > 4) throw new Error("Too many IP parts");

    let num = new Uint32Array(1)[0];
    for (let i = 0; i < 4; i++) {
        if (parts[i] == '' || isNaN(parts[i]) || parts[i] < 0 || parts[i] > 255) {
            throw new Error("Invalid IP part found");
        }
        num |= (parts[i] << (8 * (3 - i)));
    }

    if (parts[0] == 10 ||
        (parts[0] == 172 && parts[1] >= 16 && parts[1] <= 31) ||
        (parts[0] == 192 && parts[1] == 168))
        throw new Error("Private addresses are not allowed");

    if (num === 0 || num === 4294967296 || num === 2130706433 || num === 2851995905)
        throw new Error("The IP address is not allowed");
}