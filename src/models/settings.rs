#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings
{
    /// The IP address of the lobby server
    pub address: String,
    /// The port of the lobby server
    pub port: u16,
    /// The password
    pub password: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsData
{
    /// The IP address of the lobby server
    pub address: u32,
    /// The port of the lobby server
    pub port: u16,
    /// The password
    pub password: String,
}

impl Settings
{
    pub fn validate_convert(&self) -> (bool, SettingsData)
    {
        if self.port == 0
        {
            return (false, SettingsData::default());
        }

        let ip = Settings::valid_ip(&self.address);
        if ip.0 == false
        {
            return (false, SettingsData::default());
        }

        if !Settings::validate_password(&self.password)
        {
            return (false, SettingsData::default());
        }

        return (
            true,
            SettingsData {
                address: ip.1,
                port: self.port,
                password: self.password.clone(),
            },
        );
    }
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
    fn valid_ip(ip: &String) -> (bool, u32)
    {
        if ip.len() == 0
        {
            return (false, 0);
        }

        let parts = ip.split(".").collect::<Vec<&str>>();
        let count = parts.len();
        if count != 4
        {
            return (false, 0);
        }

        let mut num: u32 = 0;
        let mut i: u32 = 0;
        for part in parts.clone()
        {
            let part = String::from(part);
            if part.len() != 1
            {
                return (false, 0);
            }

            let part = part.as_bytes()[0] as u32;
            num |= (part << (8 * (3 - i)));
            i += 1;
        }

        let first = parts[0].as_bytes()[0] as u8 - '0' as u8;
        let second = parts[1].as_bytes()[0] as u8 - '0' as u8;
        if first == 10
            || (first == 172 && second >= 16 && second <= 31)
            || (first == 192 && second == 168)
        {
            return (false, 0);
        }

        // 0.0.0.0 / 255.255.255.255 / localhost / router
        if num == 0 || num == u32::MAX || num == 2130706433 || num == 2851995905
        {
            return (false, 0);
        }

        return (true, num);
    }
}
