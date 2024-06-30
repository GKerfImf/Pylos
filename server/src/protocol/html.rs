use crate::game::client::UserUUID;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterRequest {
    pub user_name: String,
    pub user_uuid: UserUUID,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterResponse {
    pub url: String,
}
