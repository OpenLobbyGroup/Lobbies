use std::{cmp::Ordering, collections::HashSet};

use crate::models::{
    app_state::AppState,
    game::Game,
    lobby::Lobby,
    query::Query,
    settings::{self, Settings},
};
use actix_web::{post, web, HttpResponse, Responder};

/// Sends lobby data, returns 401 when lobby doesn't exist
#[post("/lobbies/{name}")]
pub async fn lobbies(state: web::Data<AppState>, name: web::Path<String>) -> impl Responder
{
    let db = state.lobbies.lock().unwrap();
    let lobby = db.get(&*name);
    if let Some(lobby) = lobby
    {
        let lobby = Lobby {
            game: Game {
                brand: lobby.game.brand.clone(),
                name: lobby.game.name.clone(),
                short_desc: lobby.game.short_desc.clone(),
                long_desc: lobby.game.long_desc.clone(),
            },
            settings: Settings {
                address: String::new(),
                port: 0,
                password: String::new(),
            },
        };
        return HttpResponse::Ok().json(lobby);
    }
    else
    {
        return HttpResponse::Unauthorized().body("Unable to locate that");
    }
}

/// Sends lobby connection data, returns 401 when lobby doesn't exist or password is incorrect
/// TODO: OL client
#[post("/join/{lobby}/{password}")]
pub async fn join(state: web::Data<AppState>, lobby: web::Path<(String, String)>)
    -> impl Responder
{
    let msg = format!(
        "Hello, this page isn't ready yet, but here is what was received! lobby - {:?}",
        lobby
    );
    HttpResponse::InternalServerError().body(msg)
}

/// Decrement the lobby's player count, returns 401 when lobby doesn't exist
/// TODO: OL client
#[post("/leave/{lobby}")]
pub async fn leave(state: web::Data<AppState>, lobby: web::Path<String>) -> impl Responder
{
    let msg = format!(
        "Hello, this page isn't ready yet, but here is what was received! lobby - {:?}",
        lobby
    );
    HttpResponse::InternalServerError().body(msg)
}

/// Sends list of lobbies, returns 401 when query is invalid
#[post("/query")]
pub async fn query(state: web::Data<AppState>, pram: web::Json<Query>) -> impl Responder
{
    let query_tags = Query::get_tags(&pram.name);
    let map = state.tagged_lobbies.lock().unwrap();
    let mut result = Vec::new();
    for tag in query_tags.iter()
    {
        if let Some(list) = map.get(&tag.to_string())
        {
            result.append(&mut list.clone());
        }
    }

    // sort the result by similarity to query name
    let tags_q = query_tags;
    result.sort_by(|x, y| {
        // get all words in x, y, and the queried name
        let tags_x = Query::get_tags(&x);
        let tags_y = Query::get_tags(&y);

        // allocate words into a list
        let mut words = Vec::with_capacity(12); // 12 words is the max
        words.append(tags_x.clone().as_mut());
        words.append(tags_y.clone().as_mut());
        words.append(tags_q.clone().as_mut());

        let mut bm_x = 0 as u16;
        let mut bm_y = 0 as u16;
        let mut bm_q = 0 as u16;

        let words_count = tags_x.len() + tags_y.len() + tags_q.len();
        for i in 0..words_count
        {
            let word = &words[i];
            bm_x |= (1 << i) & (tags_x.contains(word) as u16);
            bm_y |= (1 << i) & (tags_y.contains(word) as u16);
            bm_q |= (1 << i) & (tags_q.contains(word) as u16);
        }

        let x_count = u16::count_ones(bm_x & bm_q);
        let y_count = u16::count_ones(bm_y & bm_q);
        if x_count > y_count
        {
            return Ordering::Greater;
        }
        else if y_count > x_count
        {
            return Ordering::Less;
        }
        else
        {
            return Ordering::Equal;
        }
    });
    HttpResponse::Ok().json(result)
}
