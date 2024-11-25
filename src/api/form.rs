use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Timelike, Utc};

use crate::{api::helper::decode_user, models::{app_state::AppState, lobby::{Lobby, LobbyData}, query::Query, status::Status, utc::UTC}};

/// Registers a lobby, returns 401 when lobby data is invalid or already exists
#[post("/create")]
pub async fn create(state: web::Data<AppState>, req: HttpRequest, lobby: web::Json<Lobby>) -> impl Responder
{
    // validate game, settings, and sesh
    let user = decode_user(&req);
    if !user.is_some()
    {
        return HttpResponse::Unauthorized().body("User data not found");
    }
    if !lobby.game.is_valid()
    {
        return HttpResponse::Unauthorized().body("Unable to verify game");
    }
    // check its not already created
    if state.lobbies.lock().unwrap().contains_key(&lobby.game.name)
    {
        return HttpResponse::Unauthorized().body("A lobby with that name already exists, choose a different name!");
    }

    let settings = lobby.settings.validate_convert();
    if !settings.0
    {
        return HttpResponse::Unauthorized().body("Unable to verify settings");
    }

    // insert the lobby tags into the db
    let tags = Query::get_tags(&lobby.game.name);
    let mut tagged = state.tagged_lobbies.lock().unwrap();
    for tag in tags.iter()
    {
        if let Some(list) = tagged.get_mut(tag)
        {
            list.push(lobby.game.name.clone());
        }
        else
        {
            let mut list = Vec::new();
            list.push(lobby.game.name.clone());
            tagged.insert(tag.to_string(), list);
        }
    }

    // add lobby to lobbies DB
    let time = Utc::now().hour().to_string() + ":" + &Utc::now().minute().to_string();
    let status = Status { player_count: 0, last_contact: UTC::new(time), rating: 0 };
    let lobby = LobbyData { game: lobby.game.clone(), settings: settings.1, status };
    state.lobbies.lock().unwrap().insert(lobby.game.name.clone(), lobby);

    // exit with ok
    HttpResponse::Ok().body("Lobby has been created")
}

/// Deletes a lobby, returns 401 when lobby doesn't exist, user is invalid or doesn't own the lobby
#[post("/delete")]
pub async fn delete(state: web::Data<AppState>, req: HttpRequest, lobby: web::Path<String>) -> impl Responder
{
    HttpResponse::InternalServerError().body("Hello, this page isn't ready yet")
}

/// Updates a lobby, returns 401 when lobby doesn't exist, user is invalid or doesn't own the lobby
#[post("/update")]
pub async fn update(state: web::Data<AppState>, req: HttpRequest, lobby: web::Json<Lobby>) -> impl Responder
{
    HttpResponse::InternalServerError().body("Hello, this page isn't ready yet")
}