use std::{fmt, str::FromStr};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserUUID(String);

impl UserUUID {
    pub fn new(uuid: String) -> Result<Self, &'static str> {
        if uuid.len() != 36 {
            return Err("Invalid UUID format");
        }
        Ok(UserUUID(uuid))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserUUID {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UserUUID::new(s.to_string())
    }
}
