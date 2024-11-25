use core::hash;
use core::hash::Hash;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, serde::Deserialize, Default)]
pub struct User
{
    pub name: String,
    pub password: String,
}
impl User
{
    pub fn is_valid(&self) -> bool { return User::validate_password(&self.password) && User::validate_username(&self.name); }

    fn validate_password(pw: &str) -> bool
    {
        // Check length (8-16 characters)
        let count = pw.chars().count();
        if count < 8 || count > 16
        {
            return false;
        }

        // Requirements
        let mut upper = false;
        let mut lower = false;
        let mut num = false;
        let mut symbol = false;
        pw.chars().for_each(|x| {
            upper = x.is_uppercase() || upper;
            lower = x.is_lowercase() || lower;
            num = x.is_numeric() || num;
            symbol = matches!(x, '!'..='/') || symbol;
        });
        return upper && lower && num && symbol;
    }
    fn validate_username(un: &String) -> bool
    {
        // Check length (5-9 characters)
        let count = un.chars().count();
        if count < 5 || count > 9
        {
            return false;
        }

        // Requirements
        return un.chars().all(|x| x.is_alphanumeric() || x == '_');
    }

    pub fn as_json(&self) -> String {
        format!(
            r#"{{"name": "{}", "password": "{}"}}"#,
            self.name, self.password
        )
    }

}
#[derive(Clone, Debug, Serialize, serde::Deserialize)]
pub struct UserData
{
    pub name: String,
    pub password: String,
    pub salt: [u8; 16],
    pub lobbies: Vec<usize>,
}

impl Eq for UserData {}

impl PartialEq for UserData
{
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}

impl Hash for UserData
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.name.hash(state); }
}

impl UserData
{
    pub const HASH_COST: u32 = 10;
}
