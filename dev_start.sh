#!/bin/bash
docker run --name mysql --rm -e MYSQL_ALLOW_EMPTY_PASSWORD=true -p 3306:3306 -d mysql
echo "Let the DB boot..."
sleep 20
DATABASE_URL=mysql://root@127.0.0.1:3306/rustacean_web diesel setup
docker cp seed.sql mysql:/seed.sql
docker exec -i mysql mysql -u root < seed.sql

# This will make a prod version of the frontend for. Run npm start in another window for file watching
cd app && npm run compile
cargo run
