use serde::Deserialize;
use lambda_runtime::{error::HandlerError, lambda, Context};
use std::str::from_utf8;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
struct Event {
    pub records: Vec<Record>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Record {
    #[serde(rename = "eventID")]
    event_id: String,
    event_version: String,
    kinesis: Kinesis,
    invoke_identity_arn: String,
    event_name: String,
    #[serde(rename = "eventSourceARN")]
    event_source_arn: String,
    event_source: String,
    aws_region: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Kinesis {
    approximate_arrival_timestamp: f64,
    partition_key: String,
    // base64 encoded
    data: String,
    kinesis_schema_version: String,
    sequence_number: String,
}

impl Kinesis {
    /// base64 decode data
    fn decoded_data(&self) -> Result<String, base64::DecodeError> {
        base64::decode(&self.data)
            .map(|bytes| from_utf8(&bytes).unwrap_or_default().into())
    }
}

fn main() {
    lambda!(handler)
}

fn handler(
    event: Event,
    _: Context,
) -> Result<(), HandlerError> {
    for record in event.records {
            println!(
                "{}",
                record
                    .kinesis
                    .decoded_data()
                    .unwrap_or_else(|_| "failed to decode record.data".into())
            );
        }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn handler_handles() {
       let event = Event {
            records: vec![
                Record {
                    kinesis: Kinesis {
                        ..Kinesis::default()
                    },
                    ..Record::default()
                }
            ],
        };
        // assert_eq!(
        //     handler(event.clone(), Context::default()).expect("expected Ok(_) value"),
        //     event
        // )
    }
}