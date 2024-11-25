use actix_web::{cookie::{time::{Duration, OffsetDateTime}, Cookie, Expiration}, HttpRequest};

use crate::models::user::User;

pub fn encode_user(user: &User) -> Cookie<'static>
{
    let exp = Expiration::DateTime(OffsetDateTime::now_utc() + Duration::weeks(1));
    let mut value = user.as_json();

    // encrypt value someday

    Cookie::build("user", value).expires(exp).http_only(true).secure(true).path("/").finish()
}

pub fn decode_user(req: &HttpRequest) -> Option<User>
{
    let mut cookie = req.cookie("sesh");
    if !cookie.is_some()
    {
        cookie = req.cookie("user");
    }
    if let Some(json) = cookie
    {
        let user = serde_json::from_str::<User>(json.value());
        return user.ok();
    }

    None
}