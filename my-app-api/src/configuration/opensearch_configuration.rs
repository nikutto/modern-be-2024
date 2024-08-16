use opensearch::{http::transport::Transport, OpenSearch};


pub fn get_opensearch_client() -> OpenSearch{
  let transport = Transport::single_node("http://127.0.0.1:9200").unwrap();
  let client = OpenSearch::new(transport);
  client
}
