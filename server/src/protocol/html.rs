use crate::state::user_uuid::UserUUID;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterRequest {
    pub user_name: String,
    pub user_uuid: UserUUID,
    pub user_avatar_uuid: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterResponse {
    pub url: String,
}
