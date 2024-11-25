use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct UTC
{
    pub hour: u8,
    pub minute: u8,
}

impl UTC
{
    /// Must be in Hours:Minutes format, ex 12:39 is 12:39 AM
    pub fn new(time: String) -> Self
    {
        let parts: Vec<&str> = time.split(":").collect();
        let hour: u8 = parts[0].parse().unwrap();
        let minute: u8 = parts[1].parse().unwrap();
        if hour >= 24 || minute >= 60
        {
            panic!("The given time was not in UTC range, {}:{}", hour, minute);
        }

        UTC { hour, minute }
    }
}
