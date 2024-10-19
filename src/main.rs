use std::fs;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::{AllowedRegions, Game, Lobby, Settings, Status, UTC};
mod models;

#[derive(Clone)]
struct AppState 
{
    lobbies: Arc<Mutex<HashSet<Lobby>>>,
}

#[get("/Total")]
async fn total(app_state: web::Data<AppState>) -> impl Responder 
{
    HttpResponse::Ok().json(app_state.lobbies.lock().unwrap().len())
    // Add: args of search parameter
    // Add: query DB with args and return the result
}

#[get("/View")]
async fn view() -> impl Responder 
{
    let page = fs::read("./Pages/index.html").unwrap();
    HttpResponse::Ok().body(page)
}

#[post("/Add")]
async fn add(app_state: web::Data<AppState>) -> impl Responder
{
    app_state.lobbies.lock().unwrap().insert
    (
        Lobby
        {
            game: Game
            {
                name: String::from("Minecraft"),
                description: String::from("Mine, Craft, Build"),
                banner: String::from("./TestImage"),
                icon: String::from("./TestImage")
            },
            settings: Settings
            {
                address: 0,
                port: 0,
                password: bcrypt::hash("TestPassword", 10).unwrap(),
                join_retries: 3,
                hidden: false,
                max_players: 10,
                start_time: UTC::new(String::from("5:00")),
                end_time: UTC::new(String::from("10:00")),
                regions: AllowedRegions
                {
                    north_america: true, europe: true, south_america: false, australia: false, africa: false, asia: false
                },
                max_ping: 100,
                idle_timeout: 15
            },
            status: Status
            { 
                player_count: 0,
                last_contact: UTC::new(String::from("0:0")),
                rating: 0
            },
            token: 12345678
        }
    );

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> 
{
    let lobbies = Arc::new(Mutex::new(HashSet::new()));
    HttpServer::new
    (
        move || 
        {
            let ad = AppState{ lobbies: Arc::clone(&lobbies) };

            App::new()
            .app_data(web::Data::new(ad))
            .service
            (
                web::scope("/OpenLobby")
                .service(total)
                .service(add)
                .service(view)
                .service(Files::new("/Pages", "Pages")) 
            )
        }
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}   