use my_app_domain::model::hello::Hello;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
  pub hits: SearchHits
}

#[derive(Debug, Deserialize)]
pub struct SearchHits {
  pub hits: Vec<SearchHit>
}

#[derive(Debug, Deserialize)]
pub struct SearchHit {
  pub _source: Hello
}
