#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserEntity {
    pub usr_user_id: i32,
    pub usr_name: String,
    pub usr_email: String,
    pub usr_password_hash: String,
}
