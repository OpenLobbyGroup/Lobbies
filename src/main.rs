use actix_files::Files;
use actix_session::{
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::Key,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use models::app_state::AppState;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("./key.pem", SslFiletype::PEM).expect("Couldn't set private key");
    builder.set_certificate_chain_file("./server.pem").expect("Couldn't set certificate");

    let key = Key::generate();

    let lobbies_db = "db/lobbies.db";
    let tagged_lobbies_db = "db/tagged_lobbies.db";
    let users_db = "db/users.db";

    let lobbies = models::util::load_from_path(&lobbies_db).await.expect("Unable to load lobbies.db");
    let lobbies: Arc<Mutex<HashMap<String, models::lobby::LobbyData>>> = Arc::new(Mutex::new(lobbies));

    let tagged_lobbies = models::util::load_from_path(&tagged_lobbies_db).await.expect("Unable to load tagged_lobbies.db");
    let tagged_lobbies: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(tagged_lobbies));

    let users = models::util::load_from_path(&users_db).await.expect("Unable to load users.db");
    let users: Arc<Mutex<HashMap<String, models::user::UserData>>> = Arc::new(Mutex::new(users));
    let ad = AppState { lobbies: Arc::clone(&lobbies), users: Arc::clone(&users), tagged_lobbies: Arc::clone(&tagged_lobbies)  };
    HttpServer::new(move || {
        App::new()
            // data
            .app_data(web::Data::new(ad.clone()))
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), key.clone()))
            
            // API
            .service(api::form::create)
            .service(api::form::delete)
            .service(api::form::update)
            .service(api::auth::signup)
            .service(api::auth::login)
            .service(api::auth::profile)
            .service(api::auth::set_session)
            .service(api::actions::lobbies)
            .service(api::actions::join)
            .service(api::actions::leave)
            .service(api::actions::query)

            // Get API
            .service(get_page_files("create"))
            .service(get_page_files("login"))
            .service(get_page_files("signup"))
            .service(get_page_files("search"))
            .service(get_page_files("lobby"))

            // resources
            .service(Files::new("/fonts", "./res/fonts"))
            .service(Files::new("/util", "./res/util"))
            // Default
            .service(default)
    })
    .bind_openssl("0.0.0.0:443", builder)
    .expect("Couldn't bind to address")
    .run()
    .await
    .expect("Unable to start server");

    models::util::upload_to_path(&lobbies_db, &*lobbies.lock().unwrap()).await.expect("Unable to upload lobbies to DB");
    models::util::upload_to_path(&users_db, &*users.lock().unwrap()).await.expect("Unable to upload users to DB");
    models::util::upload_to_path(&tagged_lobbies_db, &*tagged_lobbies.lock().unwrap()).await.expect("Unable to upload tagged_lobbies to DB");

    Ok(())
}

#[actix_web::get("/")]
async fn default() -> impl Responder { HttpResponse::Found().append_header(("Location", "/home/")).finish() }

fn get_page_files(name: &str) -> Files
{
    let path = "./res/pages/".to_owned() + &name;
    let index = String::from(name) + ".html";
    Files::new(&name, path).redirect_to_slash_directory().index_file(index)
}