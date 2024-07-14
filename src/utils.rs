use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::chrono;
use sqlx::types::chrono::{DateTime, Utc};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use uuid::Uuid;

pub fn serialize_debug<T: Debug, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{:?}", value))
}

pub fn serialize_display<T: Display, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{}", value))
}

pub fn deserialize_from_str<'de, T: FromStr, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    <T as FromStr>::Err: Display,
{
    let value: String = String::deserialize(deserializer)?;
    T::from_str(&value).map_err(Error::custom)
}

pub fn deserialize_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<chrono::NaiveDateTime, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    Ok(DateTime::parse_from_rfc3339(&time)
        .map_err(Error::custom)?
        .naive_utc())
}

pub fn str_to_time(time: &str) -> Option<chrono::NaiveDateTime> {
    Some(DateTime::parse_from_rfc3339(time).ok()?.naive_utc())
}

pub fn serialize_time<S: Serializer>(
    value: &chrono::NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let string_repr = DateTime::<Utc>::from_naive_utc_and_offset(value.clone(), Utc).to_rfc3339();
    string_repr.serialize(serializer)
}

pub fn deserialize_option_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<chrono::NaiveDateTime>, D::Error> {
    let maybe_time: Option<String> = Option::deserialize(deserializer)?;
    Ok(match maybe_time {
        Some(time) => Some(
            DateTime::parse_from_rfc3339(&time)
                .map_err(Error::custom)?
                .naive_utc(),
        ),
        None => None,
    })
}

pub fn now() -> chrono::NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn serialize_uuid<S: Serializer>(value: &Uuid, serializer: S) -> Result<S::Ok, S::Error> {
    uuid::serde::simple::serialize(value, serializer)
}

pub fn deserialize_uuid<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Uuid, D::Error> {
    uuid::serde::simple::deserialize(deserializer)
}
