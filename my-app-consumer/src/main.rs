mod handler;
mod model;
use model::hello_message::HelloMessage;
use opensearch::http::transport::Transport;
use opensearch::OpenSearch;
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext};
use rdkafka::message::Message;
use rdkafka::util::get_rdkafka_version;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {}

type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(client: OpenSearch, brokers: &str, group_id: &str, topics: &[&str]) {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol", "PLAINTEXT")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                match m.payload_view::<str>() {
                    None => (),
                    Some(Ok(s)) => {
                        let message: HelloMessage = serde_json::from_str(s).unwrap();
                        let result = handler::hello_handler::handle(&client, message).await;
                        match result {
                            Ok(_) => {
                                println!("Message processed.")
                            }
                            Err(e) => {
                                println!("Error while processing message: {:?}", e)
                            }
                        }
                    }
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                    }
                };
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

#[tokio::main]
async fn main() {
    let (version_n, version_s) = get_rdkafka_version();
    println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    let topics = ["debezium-cdc.mydb.hello"];
    let brokers = "127.0.0.1:9092";
    let group_id = "my-app-consumer-group-id";

    let transport = Transport::single_node("http://127.0.0.1:9200").unwrap();
    let client = OpenSearch::new(transport);

    consume_and_print(client, brokers, group_id, &topics).await
}
