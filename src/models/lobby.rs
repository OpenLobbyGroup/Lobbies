use super::game::Game;
use super::settings::{Settings, SettingsData};
use super::status::Status;
use std::collections::HashMap;
use std::hash;
use std::hash::Hash;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Lobby
{
    pub game: Game,
    pub settings: Settings
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LobbyData
{
    pub game: Game,
    pub settings: SettingsData,
    pub status: Status,
}

impl Eq for LobbyData {}

impl PartialEq for LobbyData
{
    fn eq(&self, other: &Self) -> bool { self.game.name == other.game.name }
}

impl Hash for LobbyData
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.game.name.hash(state); }
}

impl LobbyData
{
    pub async fn load_from_path(path: &str) -> std::io::Result<std::collections::HashMap<String, Lobby>>
    {
        if std::path::Path::exists(std::path::Path::new(path))
        {
            let buf = tokio::fs::read(path).await.expect("Unable to read lobby DB path");
            let set = bincode::deserialize(&buf).expect("Unable to deserialize lobby DB buffer");
            Ok(set)
        }
        else
        {
            Ok(std::collections::HashMap::new())
        }
    }

    pub async fn upload_to_path(path: &str, set: HashMap<String, Lobby>) -> std::io::Result<()>
    {
        if !std::path::Path::exists(std::path::Path::new(path))
        {
            std::fs::create_dir(path).expect("Unable to create lobby DB path");
        }

        let buf = bincode::serialize(&set).expect("Unable to serialize lobby set");
        tokio::fs::write(path, buf).await.expect("Unable to write lobby DB path");

        Ok(())
    }
}
