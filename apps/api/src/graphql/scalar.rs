use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct GraphQLDateTime(pub DateTime<Utc>);

#[Scalar(name = "DateTime")]
impl ScalarType for GraphQLDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(value) => {
                let datetime = DateTime::parse_from_rfc3339(&value)
                    .map_err(|_| InputValueError::custom("Invalid ISO-8601 datetime"))?
                    .with_timezone(&Utc);

                Ok(Self(datetime))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339_opts(
            chrono::SecondsFormat::AutoSi,
            true,
        ))
    }
}