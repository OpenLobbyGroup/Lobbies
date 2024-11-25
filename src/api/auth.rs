use actix_web::{cookie::{Cookie, Expiration}, post, web, HttpRequest, HttpResponse, Responder};
use rand::{thread_rng, Rng};

use crate::{api::helper::encode_user, api::helper::decode_user, models::{app_state::AppState, user::{User, UserData}}};

/// Registers a user, attaches cookie if user wants to be remembered, returns 401 when user is invalid or already exists
#[post("/signup/{remeber}")]
pub async fn signup(state: web::Data<AppState>, user: web::Json<User>, remember: web::Path<bool>) -> impl Responder
{
    // exit if user input is invalid
    if !user.is_valid()
    {
        return HttpResponse::Unauthorized().body("Unable to validate user!");
    }

    // exit if user already exists
    let mut users_db = state.users.lock().unwrap();
    if users_db.contains_key(&user.name)
    {
        return HttpResponse::Unauthorized().body("You are already registered!");
    }

    // inser new user-data into db and exit
    users_db.insert(user.name.clone(), hash_user(&user));
    if *remember
    {
        let cookie = encode_user(&user.0);
        return HttpResponse::Ok().cookie(cookie).body("You have been registered!");
    }
    else
    {
        return HttpResponse::Ok().body("You have been registered!");
    }

    fn hash_user(user: &User) -> UserData
    {
        let mut salt: [u8; 16] = [0; 16];
        for i in 0..16
        {
            salt[i] = thread_rng().gen::<u8>();
        }
        let hash = bcrypt::hash_with_salt(user.password.clone(), UserData::HASH_COST, salt).unwrap();

        UserData 
        {
            name: user.name.clone(),
            password: hash.format_for_version(bcrypt::Version::TwoB),
            salt,
            lobbies: vec![0]
        }
    }
}

/// Login the user, attach cookie if user wants to be remembered, returns 401 when invalid user, doesn't exist, or incorrect password
#[post("/login/{remember}")]
pub async fn login(state: web::Data<AppState>, user: web::Json<User>, remember: web::Path<bool>) -> impl Responder
{

    // exit if user input is invalid
    if !user.is_valid()
    {
        return HttpResponse::Unauthorized().body("Unable to validate user");
    }

    // check if user exists already
    let users_db = state.users.lock().unwrap();
    if let Some(stored) = users_db.get(&user.name)
    {
        let hashed = bcrypt::hash_with_salt(&user.password, UserData::HASH_COST, stored.salt).unwrap();
        if stored.password == hashed.format_for_version(bcrypt::Version::TwoB)
        {
            if *remember
            {
                let cookie = encode_user(&user.0);
                return HttpResponse::Ok().cookie(cookie).body("You have been logged in!");
            }
            else
            {
                return HttpResponse::Ok().body("You have been logged in!");
            }
        }
        else
        {
            return HttpResponse::Unauthorized().body("Password is incorrect!");
        }
    }
    else
    {
        return HttpResponse::Unauthorized().body("No user found!");
    }
}

/// Checks if user cookie is valid, returns 401 when user is invalid, doesn't exist, or incorrect password
#[post("/profile")]
pub async fn profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder
{
    if let Some(user) = decode_user(&req)
    {
        if !user.is_valid()
        {
            return HttpResponse::Unauthorized().body("Unable to validate user!");
        }

        let users_db = state.users.lock().unwrap();
        if let Some(stored) = users_db.get(&user.name)
        {
            let hashed = bcrypt::hash_with_salt(user.password, UserData::HASH_COST, stored.salt).unwrap();
            if stored.password == hashed.format_for_version(bcrypt::Version::TwoB)
            {
                return HttpResponse::Ok().body("User found!");
            }
            else
            {
                return HttpResponse::Unauthorized().body("Password was incorrect!");
            }
        }
        else
        {
            return HttpResponse::Unauthorized().body("User not found!");
        }
    }
    else
    {
        return HttpResponse::Unauthorized().body("Unable to find session cookie!");
    }
}

/// Attaches session cookie, no reason to validate here because that is being done in every function
#[actix_web::post("/sesh")]
pub async fn set_session(user: web::Json<User>) -> impl Responder
{
    let cookie = Cookie::build("sesh", user.as_json()).expires(Expiration::Session).http_only(true).secure(true).path("/").finish();
    HttpResponse::Ok().cookie(cookie).finish()
}