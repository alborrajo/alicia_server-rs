use postgres_from_row::FromRow;

#[derive(FromRow)]
pub struct Account {
    pub member_no: u32,
    pub login_id: String,
    pub auth_key: String,
}
