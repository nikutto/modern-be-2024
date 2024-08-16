## What is this for

To learn modern BE.

## Techstacks

### Language related

- Rust

### Infra related

- MySQL
- Kafka
- Debezium
- OpenSearch

## Getting started

1. Install rustup
2. `set -a; source ./scripts/local.env; set +a;`
  - Set all local environment variable
3. `cargo run`

## Architecture

Persisted data are automatically inserted into OpenSearch using CDC.

### Data flow

my-app-api --> MySQL --> Debezium --> Kafka --> my-app-consumer --> OpenSearch

## postscript (in Japanese)

一般的なデータを永続化するシステム構成について、
RDBを一貫性やトランザクションができる中央DBとして用意して
CDCでデータ連携するシステムが綺麗だと思うのでデモを作ってみた。
Kafkaに連携しているので新しい連携先（新しい検索エンジンなど）が増えても
既存コンポーネントの変更なしに追加できて綺麗。
my-app-consumerの部分はKafka connectでOpenSearchをSinkにしてもよいが、
自作した方が取り回しが楽だと思う。

言語はコンセプトが好きなのでRustで書いてみた。
これくらいの簡単な処理だとRustだと言語観が丁寧すぎるように感じた。
Resultより古き例外のほうが雑な処理がしやすくて楽かもしれない。
途中でめんどくさくなって例外処理なんかを雑にサボっているが
デモなので大目にみてほしい。
