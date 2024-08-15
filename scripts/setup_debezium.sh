BASEDIR=$(dirname "$0")
curl -i -X POST -H "Accept:application/json" -H  "Content-Type:application/json" http://localhost:8083/connectors -d @$BASEDIR/debezium/connector.json
