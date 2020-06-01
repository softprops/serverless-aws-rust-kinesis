use aws_lambda_events::event::kinesis::KinesisEvent;
use lambda::handler_fn;
use std::str::from_utf8;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(handler)).await?;
    Ok(())
}

async fn handler(event: KinesisEvent) -> Result<(), Error> {
    for record in event.records {
        println!(
            "{}",
            from_utf8(&record.kinesis.data.0)
                .map(|s| s.to_owned())
                .unwrap_or_else(|err| format!("expected utf8 data: {}", err))
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handler_handles() {
        let event: KinesisEvent =
            serde_json::from_slice(include_bytes!("../tests/example-event.json"))
                .expect("invalid kinesis event");

        assert!(handler(event).await.is_ok())
    }
}
