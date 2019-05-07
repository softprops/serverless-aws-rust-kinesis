use aws_lambda_events::event::kinesis::KinesisEvent;
use lambda_runtime::{error::HandlerError, lambda, Context};
use std::str::from_utf8;

fn main() {
    lambda!(handler)
}

fn handler(
    event: KinesisEvent,
    _: Context,
) -> Result<(), HandlerError> {
    for record in event.records {
        println!(
            "{}",
            base64::decode(&record.kinesis.data.0)
                .map(|bytes| from_utf8(&bytes).unwrap_or_default().to_owned())
                .unwrap_or_else(|err| format!("failed to decode record.data {}", err))
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let event: KinesisEvent =
            serde_json::from_slice(include_bytes!("../tests/example-event.json"))
                .expect("invalid kinesis event");

        assert!(handler(event, Context::default()).is_ok())
    }
}
