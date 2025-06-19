use std::error::Error;

use postgres_from_row::FromRow;
use tokio_postgres::Transaction;

use crate::{
    commands::shared::horse::Horse,
    database::{CStringSql, U8Sql, U16Sql, U32Sql},
};

pub async fn get_horses_by_character_id<'a>(
    transaction: &mut Transaction<'a>,
    character_id: u32,
) -> Result<Vec<Horse>, Box<dyn Error>> {
    let rows = transaction
        .query(
            "SELECT * FROM horses WHERE character_id = $1",
            &[&character_id],
        )
        .await?;
    Ok(rows.iter().map(|row| Horse::from_row(row)).collect())
}

pub async fn get_horse_by_uid<'a>(
    transaction: &mut Transaction<'a>,
    uid: u32,
) -> Result<Option<Horse>, Box<dyn Error>> {
    let row_opt = transaction
        .query_opt("SELECT * FROM horses WHERE uid = $1", &[&U32Sql::from(uid)])
        .await?;
    if let Some(row) = row_opt {
        let horse = Horse::try_from_row(&row)?;
        Ok(Some(horse))
    } else {
        Ok(None)
    }
}

pub async fn insert_horse<'a>(
    transaction: &mut Transaction<'a>,
    character_id: u32,
    horse: &mut Horse,
) -> Result<(), Box<dyn Error>> {
    let row = transaction
        .query_one(
            "INSERT INTO horses (
                character_id,
                tid, name,
                skin_id, mane_id, tail_id, face_id,
                scale, leg_length, leg_volume, body_length, body_volume,
                agility, control, speed, strength, spirit,
                rating, class, class_progress, grade, growth_points,
                stamina, attractiveness, hunger,
                vals0_val0, vals0_val1, vals0_val2, vals0_val3, vals0_val4,
                vals0_val5, vals0_val6, vals0_val7, vals0_val8, vals0_val9, vals0_val10,
                vals1_val0, vals1_val1, date_of_birth, vals1_val3, vals1_val4,
                class_progression, vals1_val5,
                potential_level, has_potential, potential_value, vals1_val9,
                luck, has_luck, vals1_val12, fatigue, vals1_val14, emblem,
                spur_magic_count, jump_count, sliding_time, gliding_distance,
                val16, val17
            ) VALUES (
                $1,
                $2, $3,
                $4, $5, $6, $7,
                $8, $9, $10, $11, $12,
                $13, $14, $15, $16, $17,
                $18, $19, $20, $21, $22,
                $23, $24, $25,
                $26, $27, $28, $29, $30,
                $31, $32, $33, $34, $35, $36,
                $37, $38, $39, $40, $41,
                $42, $43,
                $44, $45, $46, $47,
                $48, $49, $50, $51, $52, $53,
                $54, $55, $56, $57,
                $58, $59
            )
            RETURNING uid",
            &[
                &U32Sql::from(character_id),
                &U32Sql::from(horse.tid),
                &CStringSql::from(horse.name.clone()),
                &U8Sql::from(horse.parts.skin_id),
                &U8Sql::from(horse.parts.mane_id),
                &U8Sql::from(horse.parts.tail_id),
                &U8Sql::from(horse.parts.face_id),
                &U8Sql::from(horse.appearance.scale),
                &U8Sql::from(horse.appearance.leg_length),
                &U8Sql::from(horse.appearance.leg_volume),
                &U8Sql::from(horse.appearance.body_length),
                &U8Sql::from(horse.appearance.body_volume),
                &U32Sql::from(horse.stats.agility),
                &U32Sql::from(horse.stats.control),
                &U32Sql::from(horse.stats.speed),
                &U32Sql::from(horse.stats.strength),
                &U32Sql::from(horse.stats.spirit),
                &U32Sql::from(horse.rating),
                &U8Sql::from(horse.class),
                &U8Sql::from(horse.class_progress),
                &U8Sql::from(horse.grade),
                &U16Sql::from(horse.growth_points),
                &U16Sql::from(horse.vals0.stamina),
                &U16Sql::from(horse.vals0.attractiveness),
                &U16Sql::from(horse.vals0.hunger),
                &U16Sql::from(horse.vals0.val0),
                &U16Sql::from(horse.vals0.val1),
                &U16Sql::from(horse.vals0.val2),
                &U16Sql::from(horse.vals0.val3),
                &U16Sql::from(horse.vals0.val4),
                &U16Sql::from(horse.vals0.val5),
                &U16Sql::from(horse.vals0.val6),
                &U16Sql::from(horse.vals0.val7),
                &U16Sql::from(horse.vals0.val8),
                &U16Sql::from(horse.vals0.val9),
                &U16Sql::from(horse.vals0.val10),
                &U8Sql::from(horse.vals1.val0),
                &U32Sql::from(horse.vals1.val1),
                &U32Sql::from(horse.vals1.date_of_birth),
                &U8Sql::from(horse.vals1.val3),
                &U8Sql::from(horse.vals1.val4),
                &U32Sql::from(horse.vals1.class_progression),
                &U32Sql::from(horse.vals1.val5),
                &U8Sql::from(horse.vals1.potential_level),
                &U8Sql::from(horse.vals1.has_potential),
                &U8Sql::from(horse.vals1.potential_value),
                &U8Sql::from(horse.vals1.val9),
                &U8Sql::from(horse.vals1.luck),
                &U8Sql::from(horse.vals1.has_luck),
                &U8Sql::from(horse.vals1.val12),
                &U16Sql::from(horse.vals1.fatigue),
                &U16Sql::from(horse.vals1.val14),
                &U16Sql::from(horse.vals1.emblem),
                &U32Sql::from(horse.mastery.spur_magic_count),
                &U32Sql::from(horse.mastery.jump_count),
                &U32Sql::from(horse.mastery.sliding_time),
                &U32Sql::from(horse.mastery.gliding_distance),
                &U32Sql::from(horse.val16),
                &U32Sql::from(horse.val17),
            ],
        )
        .await?;
    let uid: U32Sql = row.try_get(0)?;
    horse.uid = uid.into();
    Ok(())
}

pub async fn remove_horse<'a>(
    transaction: &mut Transaction<'a>,
    horse_uid: u32,
) -> Result<(), Box<dyn Error>> {
    let rows_affected = transaction
        .execute(
            "DELETE FROM horses WHERE uid = $1",
            &[&U32Sql::from(horse_uid)],
        )
        .await?;
    if rows_affected != 1 {
        Err(format!("More than one row affected: {}", rows_affected).into())
    } else {
        Ok(())
    }
}
