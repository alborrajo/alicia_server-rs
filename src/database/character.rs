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

pub async fn get_character_by_id<'a>(
    transaction: &mut Transaction<'a>,
    character_id: u32,
) -> Result<Option<Character>, Box<dyn Error>> {
    let row_opt = transaction
        .query_opt(
            "SELECT * FROM characters WHERE character_id = $1",
            &[&U32Sql::from(character_id)],
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
    character: &mut Character,
) -> Result<(), Box<dyn Error>> {
    let row = transaction
        .query_one(
            "INSERT INTO characters (
                member_no,
                nickname,
                mount_uid,
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
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
             RETURNING character_id",
            &[
                &U32Sql::from(member_no),
                &character.nickname,
                &U32Sql::from(character.mount_uid),
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
    let character_id: U32Sql = row.try_get(0)?;
    character.character_id = character_id.into();
    Ok(())
}

pub async fn update_character<'a>(
    transaction: &mut Transaction<'a>,
    character: &Character,
) -> Result<(), Box<dyn Error>> {
    let rows_affected = transaction
        .execute(
            "UPDATE characters SET
                nickname = $1,
                mount_uid = $2,
                char_id = $3,
                mouth_serial_id = $4,
                face_serial_id = $5,
                parts_val0 = $6,
                appearance_val0 = $7,
                head_size = $8,
                height = $9,
                thigh_volume = $10,
                leg_volume = $11,
                appearance_val1 = $12,
                create_character_unk0 = $13
            WHERE character_id = $14",
            &[
                &character.nickname,
                &U32Sql::from(character.mount_uid),
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
                &U32Sql::from(character.character_id),
            ],
        )
        .await?;
    if rows_affected == 1 {
        Ok(())
    } else {
        Err(format!("Unexpected number of rows affected: {}", rows_affected).into())
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
