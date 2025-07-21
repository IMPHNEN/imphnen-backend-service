use serde::{self, Deserialize, Deserializer, Serializer};
use surrealdb::sql::Thing;

use serde::de::{self};
use serde::ser::Serialize;
use serde_json::Value;
use std::str::FromStr;

pub fn thing_or_string<'de, D>(deserializer: D) -> Result<Thing, D::Error>
where
	D: Deserializer<'de>,
{
	let v = Value::deserialize(deserializer)?;
	match &v {
		Value::Object(map) => {
			if let Some(id_val) = map.get("Id") {
				if let Value::Object(id_map) = id_val {
					if let Some(Value::String(s)) = id_map.get("String") {
						return Thing::from_str(s).map_err(|e| {
							de::Error::custom(format!("Thing::from_str error: {e:?}"))
						});
					}
				}
			}
			serde_json::from_value(v).map_err(de::Error::custom)
		}
		Value::String(s) => {
			if s.is_empty() {
				Thing::from_str("unknown:empty")
					.map_err(|e| de::Error::custom(format!("Thing::from_str error: {e:?}")))
			} else {
				Thing::from_str(s)
					.map_err(|e| de::Error::custom(format!("Thing::from_str error: {e:?}")))
			}
		}
		_ => Err(de::Error::custom(
			"Expected SurrealDB Thing object, string, or enum Id::String",
		)),
	}
}

pub fn option_thing_or_string<'de, D>(
	deserializer: D,
) -> Result<Option<Thing>, D::Error>
where
	D: Deserializer<'de>,
{
	let v = Value::deserialize(deserializer)?;
	match &v {
		Value::Null => Ok(None),
		Value::Object(map) => {
			if let Some(id_val) = map.get("Id") {
				if let Value::Object(id_map) = id_val {
					if let Some(Value::String(s)) = id_map.get("String") {
						return Ok(Some(Thing::from_str(s).map_err(|e| {
							de::Error::custom(format!("Thing::from_str error: {e:?}"))
						})?));
					}
				}
			}
			Ok(Some(serde_json::from_value(v).map_err(de::Error::custom)?))
		}
		Value::String(s) => {
			if s.is_empty() {
				Ok(None)
			} else {
				Ok(Some(Thing::from_str(s).map_err(|e| {
					de::Error::custom(format!("Thing::from_str error: {e:?}"))
				})?))
			}
		}
		_ => Err(de::Error::custom(
			"Expected SurrealDB Thing object, string, enum Id::String, or null",
		)),
	}
}

pub fn string_or_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	let v = Value::deserialize(deserializer)?;
	match v {
		Value::String(s) => Ok(s),
		Value::Null => Ok(String::new()),
		_ => Err(de::Error::custom("Expected a string or null")),
	}
}

pub fn serialize_thing<S>(thing: &Thing, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	thing.to_string().serialize(serializer)
}

pub fn serialize_option_thing<S>(
	thing: &Option<Thing>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match thing {
		Some(t) => Some(t.to_string()).serialize(serializer),
		None => None::<String>.serialize(serializer),
	}
}

pub fn serialize_datetime<S>(
	datetime: &chrono::DateTime<chrono::Utc>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&datetime.to_rfc3339())
}

pub fn deserialize_datetime<'de, D>(
	deserializer: D,
) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	chrono::DateTime::parse_from_rfc3339(&s)
		.map_err(de::Error::custom)
		.map(|dt| dt.with_timezone(&chrono::Utc))
}
