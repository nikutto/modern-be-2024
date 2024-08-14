BASEDIR=$(dirname "$0")

mysql -u root -h 127.0.0.1 -e 'DROP DATABASE IF EXISTS mydb';
mysql -u root -h 127.0.0.1 -e 'CREATE DATABASE mydb';
for ddl in $BASEDIR/../sql/*.sql; do mysql -u root -h 127.0.0.1 mydb < $ddl; done;
