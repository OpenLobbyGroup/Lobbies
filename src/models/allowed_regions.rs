use serde::Deserialize;
use serde::Serialize;

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
