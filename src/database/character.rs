use std::error::Error;

use postgres_from_row::FromRow;
use tokio_postgres::Transaction;

use crate::{
    database::{U8Sql, U16Sql, U32Sql},
    entities::character::Character,
};

pub async fn get_character_by_member_no<'a>(
    transaction: &mut Transaction<'a>,
    member_no: u32,
) -> Result<Option<Character>, Box<dyn Error>> {
    let row_opt = transaction
        .query_opt(
            "SELECT * FROM characters WHERE member_no = $1",
            &[&U32Sql::from(member_no)],
        )
        .await?;
    if let Some(row) = row_opt {
        let character = Character::try_from_row(&row)?;
        Ok(Some(character))
    } else {
        Ok(None)
    }
}

pub async fn insert_character<'a>(
    transaction: &mut Transaction<'a>,
    member_no: u32,
    character: &Character,
) -> Result<(), Box<dyn Error>> {
    let rows = transaction
        .execute(
            "INSERT INTO characters (
                member_no,
                character_id,
                nickname,
                char_id,
                mouth_serial_id,
                face_serial_id,
                parts_val0,
                appearance_val0,
                head_size,
                height,
                thigh_volume,
                leg_volume,
                appearance_val1,
                create_character_unk0
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)",
            &[
                &U32Sql::from(member_no),
                &U32Sql::from(character.character_id),
                &character.nickname,
                &U8Sql::from(character.character.parts.char_id),
                &U8Sql::from(character.character.parts.mouth_serial_id),
                &U8Sql::from(character.character.parts.face_serial_id),
                &U8Sql::from(character.character.parts.val0),
                &U16Sql::from(character.character.appearance.val0),
                &U16Sql::from(character.character.appearance.head_size),
                &U16Sql::from(character.character.appearance.height),
                &U16Sql::from(character.character.appearance.thigh_volume),
                &U16Sql::from(character.character.appearance.leg_volume),
                &U16Sql::from(character.character.appearance.val1),
                &U32Sql::from(character.create_character_unk0),
            ],
        )
        .await?;
    if rows == 1 {
        Ok(())
    } else {
        Err(format!("Unexpected number of rows affected: {}", rows).into())
    }
}

pub async fn delete_character<'a>(
    transaction: &mut Transaction<'a>,
    character_id: u32,
) -> Result<(), Box<dyn Error>> {
    let rows = transaction
        .execute(
            "DELETE FROM characters WHERE character_id = $1",
            &[&character_id],
        )
        .await?;
    if rows == 1 {
        Ok(())
    } else {
        Err(format!("Unexpected number of rows affected: {}", rows).into())
    }
}
