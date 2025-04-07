use actix_web::rt::task::JoinHandle;

use crate::db::{
    DatabaseConnection,
    types::{DB_Account, DB_MonthlyFee, DB_Role, ID},
};

use super::{Account, MonthlyFee, Payments, Role};

pub async fn create_account(db: DatabaseConnection, account: DB_Account) -> anyhow::Result<bool> {
    match sqlx::query(
        r"
            INSERT INTO
            accounts(
                phone_number, name, lastname, email, hashed_password, password_set_ts
            )
            VALUES (
                $1, $2, $3, $4, $5, $6
            );
        ",
    )
    .bind(&account.phone_number)
    .bind(&account.name)
    .bind(&account.lastname)
    .bind(&account.email)
    .bind(&account.hashed_password)
    .bind(&account.password_set_ts)
    .execute(&db)
    .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Ok(false),
        Err(err) => Err(err)?,
    }
}

pub async fn create_roles(db: DatabaseConnection, roles: Vec<String>) -> anyhow::Result<bool> {
    match sqlx::query(
        r"
            INSERT INTO
            roles(
                role
            )
            SELECT * FROM UNNEST(
                $1::text[]
            );
        ",
    )
    .bind(&roles)
    .execute(&db)
    .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Ok(false),
        Err(err) => Err(err)?,
    }
}

pub async fn add_roles_to_account(
    db: DatabaseConnection,
    account_id: ID,
    role_ids: Vec<ID>,
) -> anyhow::Result<bool> {
    match sqlx::query(
        r"
            INSERT INTO
            account_roles(
                account_id, role_id
            )
            SELECT * FROM UNNEST(
                $1::int[], $2::int[]
            );
        ",
    )
    .bind((0..role_ids.len()).map(|_| account_id).collect::<Vec<_>>())
    .bind(&role_ids)
    .execute(&db)
    .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Ok(false),
        Err(err) => Err(err)?,
    }
}

pub async fn fetch_account_by_email(
    db: DatabaseConnection,
    email: String,
) -> anyhow::Result<Option<Account>> {
    let account: Option<DB_Account> = match sqlx::query_as(
        r"
            SELECT * FROM accounts
            WHERE email = $1;
        ",
    )
    .bind(email)
    .fetch_one(&db)
    .await
    {
        Ok(a) => Some(a),
        Err(sqlx::Error::RowNotFound) => None,
        Err(err) => Err(err)?,
    };

    Ok(account.map(|a| Account {
        id: a.id,
        phone_number: a.phone_number,
        name: a.name,
        lastname: a.lastname,
        email: a.email,
        hashed_password: a.hashed_password,
        password_set_ts: a.password_set_ts as u64,
    }))
}

pub async fn fetch_account_by_phone_number(
    db: DatabaseConnection,
    phone_number: String,
) -> anyhow::Result<Option<Account>> {
    let account: Option<DB_Account> = match sqlx::query_as(
        r"
            SELECT * FROM accounts
            WHERE phone_number = $1;
        ",
    )
    .bind(phone_number)
    .fetch_one(&db)
    .await
    {
        Ok(a) => Some(a),
        Err(sqlx::Error::RowNotFound) => None,
        Err(err) => Err(err)?,
    };

    Ok(account.map(|a| Account {
        id: a.id,
        phone_number: a.phone_number,
        name: a.name,
        lastname: a.lastname,
        email: a.email,
        hashed_password: a.hashed_password,
        password_set_ts: a.password_set_ts as u64,
    }))
}

pub async fn fetch_account_roles(
    db: DatabaseConnection,
    account_id: ID,
) -> anyhow::Result<Vec<Role>> {
    let roles: Vec<DB_Role> = sqlx::query_as(
        r"
            SELECT
                roles.id,
                roles.role
            FROM account_roles
            JOIN roles ON roles.id = account_roles.role_id
                AND account_roles.account_id = $1;
        ",
    )
    .bind(account_id)
    .fetch_all(&db)
    .await?;

    Ok(roles.into_iter().map(|r| Role { role: r.role }).collect())
}

pub async fn fetch_account_payments(
    db: DatabaseConnection,
    account_id: ID,
) -> anyhow::Result<Payments> {
    let join_payment_rows: JoinHandle<Result<Vec<DB_MonthlyFee>, _>> = {
        let db = db.clone();
        actix_web::rt::spawn(async move {
            sqlx::query_as(
                r"
                    SELECT
                        monthly_fees.id,
                        monthly_fees.year,
                        monthly_fees.month
                    FROM payments
                    JOIN monthly_fees ON monthly_fees.id = payments.fee_id;
                        AND payments.account_id = $1
                ",
            )
            .bind(account_id)
            .fetch_all(&db)
            .await
        })
    };

    let join_pre_payment_rows: JoinHandle<Result<Vec<DB_MonthlyFee>, _>> = {
        let db = db.clone();
        actix_web::rt::spawn(async move {
            sqlx::query_as(
                r"
                    SELECT
                        pre_payments.id,
                        pre_payments.year,
                        pre_payments.month
                    FROM pre_payments
                    WHERE pre_payments.account_id = $1;
                ",
            )
            .bind(account_id)
            .fetch_all(&db)
            .await
        })
    };

    Ok(Payments {
        made: join_payment_rows
            .await??
            .into_iter()
            .map(|m| MonthlyFee {
                year: m.year as u32,
                month: m.month as u32,
            })
            .collect(),
        precovered: join_pre_payment_rows
            .await??
            .into_iter()
            .map(|m| MonthlyFee {
                year: m.year as u32,
                month: m.month as u32,
            })
            .collect(),
    })
}
