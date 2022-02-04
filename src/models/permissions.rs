use std::io;

use diesel::{
    backend::Backend,
    deserialize,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};

use crate::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, Serialize, Deserialize, FromSqlRow)]
#[sql_type = "diesel::sql_types::Integer"]
pub enum Permission {
    Guest,
    User,
    AuthAuthor,
    Admin,
}

impl<DB: Backend> ToSql<Integer, DB> for Permission
where
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        let v = match *self {
            Permission::Guest => 0,
            Permission::User => 1,
            Permission::AuthAuthor => 2,
            Permission::Admin => 3,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Integer, DB> for Permission
where
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = i32::from_sql(bytes)?;
        Ok(match v {
            0 => Permission::Guest,
            1 => Permission::User,
            2 => Permission::AuthAuthor,
            3 => Permission::Admin,
            _ => return Err("Invalid permission".into()),
        })
    }
}
