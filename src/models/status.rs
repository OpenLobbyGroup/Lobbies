use crate::models::utc::UTC;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Status
{
    pub player_count: u8,
    /// The last time the server sent a message
    pub last_contact: UTC,
    pub rating: u8,
}
