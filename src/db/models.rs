use super::schema::posts;

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct PostShort {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub caption: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String
}
