use anyhow::Ok;
use opensearch::OpenSearch;

use crate::model::hello_message::HelloMessage;

pub async fn handle(client: &OpenSearch, event: HelloMessage) -> anyhow::Result<()> {
    match event.payload.after {
        Some(after) => {
            let resp = client
                .index(opensearch::IndexParts::IndexId(
                    "hello",
                    &after.id.to_string(),
                ))
                .body(after)
                .send()
                .await?;
            if resp.status_code().is_success() {
                Ok(())
            } else {
                let bytes = resp.bytes().await.unwrap();
                let error_msg = std::str::from_utf8(&bytes)?.to_string();
                Err(anyhow::anyhow!(error_msg))
            }
        }
        None => Err(anyhow::anyhow!("Unexpected")),
    }
}
