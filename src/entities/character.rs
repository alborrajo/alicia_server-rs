use postgres_from_row::FromRow;

use crate::database::U32Sql;

#[derive(Clone, FromRow)]
pub struct Character {
    #[from_row(from = "U32Sql")]
    pub character_id: u32,
    pub nickname: String,
    #[from_row(flatten)]
    pub character: crate::commands::shared::character::Character,
    #[from_row(from = "U32Sql")]
    pub create_character_unk0: u32,
}
