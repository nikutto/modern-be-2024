use my_app_domain::model::hello::Hello;
use mysql::serde_json::json;

use crate::{model::search_response::SearchResponse, repository::hello_repository::{insert_hello, select_hello}};

pub fn get_hello(pool: &mysql::Pool, id: u64) -> Result<Option<Hello>, mysql::error::Error> {
    let hello = select_hello(pool, id)?;
    match hello {
        Some(hello) => Ok(Some(hello)),
        None => Ok(None),
    }
}

pub fn create_hello(pool: &mysql::Pool, msg: &String) -> Result<u64, mysql::error::Error> {
    let id = insert_hello(pool, msg)?;
    Ok(id)
}

pub async fn search_hello(client: &opensearch::OpenSearch, pool: &mysql::Pool, query: &String) -> anyhow::Result<Vec<Hello>> {
    let indices = ["hello"];
    let response = client
        .search(
            opensearch::SearchParts::Index(&indices)
        )
        .body(json!({
            "query": {
                "match": {
                    "message": query
                }
            }
        }))
        .send()
        .await?;
    if response.status_code().is_success() {
        let result = response.json::<SearchResponse>().await;
        match result {
            Ok(result) => {
                let ids = result.hits.hits.into_iter().map(|hit| hit._source.id);
                let hellos: Vec<Result<Option<Hello>, mysql::Error>> = ids.map(|id| get_hello(&pool, id)).collect();
                let is_err = hellos.iter().any (|hello| hello.is_err().clone());
                if is_err {
                    let errors = hellos.iter().filter(|hello| hello.is_err()).collect::<Vec<&Result<Option<Hello>, mysql::Error>>>();
                    return Err(anyhow::anyhow!("Failed to fetch data from mysql. {:?}", errors));
                }
                let hellos = hellos
                    .into_iter()
                    .filter_map(
                        |hello| match hello {
                            Ok(Some(h)) => Some(h),
                            _ => None,
                    })
                    .collect::<Vec<Hello>>();
                Ok(hellos)
            },
            Err(e) => {
                Err(anyhow::anyhow!("Failed to search data. {:?}", e))
            }
        }
        
    }
    else {
        Err(anyhow::anyhow!("Failed to search data. {:?}", response.json().await?))
    }
}
