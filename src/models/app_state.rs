use super::{lobby::LobbyData, user::UserData};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct AppState
{
    pub lobbies: Arc<Mutex<HashMap<String, LobbyData>>>,
    pub users: Arc<Mutex<HashMap<String, UserData>>>,
    pub tagged_lobbies: Arc<Mutex<HashMap<String, Vec<String>>>>,
}
