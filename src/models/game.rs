use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Game
{
    pub brand: String,      // ex Minecraft
    pub name: String,       // ex My Survial Server
    pub short_desc: String, // displayed on the card
    pub long_desc: String,  // displayed on full view
}

impl Game
{
    pub fn is_valid(&self) -> bool
    {
        return in_range(&self.brand, 4, 9)
            && in_range(&self.name, 4, 9)
            && in_range(&self.short_desc, 40, 120)
            && in_range(&self.long_desc, 0, 1000);

        fn in_range(str: &String, min: u32, max: u32) -> bool
        {
            let count = str.chars().count() as u32;
            return count >= min && count <= max;
        }
    }
}
