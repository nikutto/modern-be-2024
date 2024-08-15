BASEDIR=$(dirname "$0")
FILE_PATH=$BASEDIR/../debezium/connector.json

curl -i -X POST -H "Accept:application/json" -H  "Content-Type:application/json" http://localhost:8083/connectors -d "@$FILE_PATH"
