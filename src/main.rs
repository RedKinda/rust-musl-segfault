use std::hint::black_box;

use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{stream_consumer::StreamConsumer, ConsumerContext},
    ClientConfig, ClientContext,
};

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}
type LoggingConsumer = StreamConsumer<CustomContext>;

#[tokio::main]
pub async fn main() {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            "example.kafka.us-east-1.amazonaws.com:9092",
        )
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("sasl.username", "waaa")
        .set("sasl.password", "weeee")
        .set("group.id", "warp")
        .set("auto.offset.reset", "latest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    let a = black_box(consumer);
    ()
}
