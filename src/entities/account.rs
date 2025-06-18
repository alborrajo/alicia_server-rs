use postgres_from_row::FromRow;

use crate::database::U32Sql;

#[derive(FromRow)]
pub struct Account {
    #[from_row(from = "U32Sql")]
    pub member_no: u32,
    pub login_id: String,
    pub auth_key: String,
}
