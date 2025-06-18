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
) -> Result<Account, Box<dyn Error>> {
    let row = transaction
        .query_one(
            "SELECT * FROM accounts WHERE member_no = $1",
            &[&U32Sql::from(member_no)],
        )
        .await?;
    let account = Account::try_from_row(&row)?;
    Ok(account)
}
pub async fn add_account<'a>(
    transaction: &mut Transaction<'a>,
    new_account: &Account,
) -> Result<(), Box<dyn Error>> {
    let rows = transaction
        .execute(
            "INSERT INTO accounts (member_no,login_id,auth_key) VALUES ($1,$2,$3)",
            &[
                &U32Sql::from(new_account.member_no),
                &new_account.login_id,
                &new_account.auth_key,
            ],
        )
        .await?;
    if rows == 1 {
        Ok(())
    } else {
        Err(format!("Unexpected number of rows affected: {}", rows).into())
    }
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
