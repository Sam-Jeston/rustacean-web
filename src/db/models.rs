#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}
