use core::hash;
use std::hash::{Hash};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Game
{
    pub name: String,
    pub description: String,
    pub banner: String,
    pub icon: String,
}

impl Game
{
    fn new(name: String, desc: String, banner_path: String, icon_path: String) -> Self
    {
        Game {
            name,
            description: desc,
            banner: banner_path,
            icon: icon_path,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Settings
{
    /// The IP address of the lobby server
    pub address: u32,
    /// The port of the lobby server
    pub port: u16,
    /// The password hash
    pub password: String,
    /// The number of times password can be retried
    pub join_retries: u8,
    /// Is the lobby only accessible form URL?
    pub hidden: bool,
    /// Maximium number of player
    pub max_players: u8,
    /// UTC time for when the lobby starts
    pub start_time: UTC,
    /// UTC time for when the lobby ends
    pub end_time: UTC,
    /// Regions that are allowed to join this lobby
    pub regions: AllowedRegions,
    /// Maximum ping allowed to join lobby, max 255
    pub max_ping: u8,
    /// How long in minutes the lobby is allowed to be idle before it is told to shut down, max 4:15
    pub idle_timeout: u8,
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Status
{
    pub player_count: u8,
    /// The last time the server sent a message
    pub last_contact: UTC,
    pub rating: u8,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Lobby
{
    pub game: Game,
    pub settings: Settings,
    pub status: Status,
    pub token: usize,
}

impl Hash for Lobby
{
    fn hash<H: hash::Hasher>(&self, state: &mut H)
    {
        self.token.hash(state);
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct UTC
{
    pub hour: u8,
    pub minute: u8,
}

impl UTC
{
    pub fn new(time: String) -> Self
    {
        let parts: Vec<&str> = time.split(":").collect();
        let hour: u8 = parts[0].parse().unwrap();
        let minute: u8 = parts[1].parse().unwrap();
        if hour >= 24 || minute >= 60 {
            panic!("The given time was not in UTC range, {}:{}", hour, minute);
        }

        UTC { hour, minute }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct AllowedRegions
{
    pub north_america: bool,
    pub south_america: bool,
    pub australia: bool,
    pub europe: bool,
    pub africa: bool,
    pub asia: bool,
}
