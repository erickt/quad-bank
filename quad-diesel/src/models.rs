use super::schema::accounts;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub balance: i32,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub balance: i32,
}
