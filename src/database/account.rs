use std::error::Error;

use postgres_from_row::FromRow;
use tokio_postgres::Transaction;

use crate::{database::U32Sql, entities::account::Account};

pub async fn get_accounts<'a>(
    transaction: &mut Transaction<'a>,
) -> Result<Vec<Account>, Box<dyn Error>> {
    let rows = transaction.query("SELECT * FROM accounts", &[]).await?;
    Ok(rows.iter().map(|row| Account::from_row(&row)).collect())
}

pub async fn get_account<'a>(
    transaction: &mut Transaction<'a>,
    member_no: u32,
) -> Result<Option<Account>, Box<dyn Error>> {
    let row = transaction
        .query_opt(
            "SELECT * FROM accounts WHERE member_no = $1",
            &[&U32Sql::from(member_no)],
        )
        .await?;
    if let Some(row) = row {
        Ok(Some(Account::try_from_row(&row)?))
    } else {
        Ok(None)
    }
}
pub async fn add_account<'a>(
    transaction: &mut Transaction<'a>,
    new_account: &mut Account,
) -> Result<(), Box<dyn Error>> {
    let row = transaction
        .query_one(
            "INSERT INTO accounts (login_id,auth_key) VALUES ($1,$2) RETURNING member_no",
            &[&new_account.login_id, &new_account.auth_key],
        )
        .await?;
    let member_no: U32Sql = row.get(0);
    new_account.member_no = member_no.into();
    Ok(())
}

pub async fn delete_account<'a>(
    transaction: &mut Transaction<'a>,
    member_no: u32,
) -> Result<(), Box<dyn Error>> {
    let rows = transaction
        .execute(
            "DELETE FROM accounts WHERE member_no = $1",
            &[&U32Sql::from(member_no)],
        )
        .await?;
    if rows == 1 {
        Ok(())
    } else {
        Err(format!("Unexpected number of rows affected: {}", rows).into())
    }
}
