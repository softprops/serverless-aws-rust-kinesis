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
            from_utf8(&record.kinesis.data.0).map(|s|s.to_owned())
                .unwrap_or_else(|err| format!("expected utf8 data: {}", err))
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
