use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::Queryable,
    serialize::{self, Output, ToSql},
    sql_types::Timestamp,
    Selectable,
};
use serde::{Deserialize, Serialize};
use std::{io::Write, str::FromStr};

// Define new types
#[derive(Clone, Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Timestamp)]
pub struct DateTimeUTC(pub DateTime<Utc>);

// Implement Scalar for DateTimeUTC
#[Scalar]
impl ScalarType for DateTimeUTC {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(DateTimeUTC(DateTime::from_str(&value).map_err(|_| {
                InputValueError::custom("Invalid DateTime format")
            })?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

// Implement FromSql for DateTimeUTC
impl FromSql<Timestamp, Pg> for DateTimeUTC {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        let s = String::from_utf8(bytes.as_bytes().to_vec())?;
        let dt = DateTime::from_str(&s)?;
        Ok(DateTimeUTC(dt))
    }
}

// Implement ToSql for DateTimeUTC
impl ToSql<Timestamp, Pg> for DateTimeUTC {
    fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Pg>) -> serialize::Result {
        let s = self.0.to_rfc3339();
        out.write_all(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
