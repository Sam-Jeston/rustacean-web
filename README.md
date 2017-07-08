# Rustacean Web

A blog platform, written with rocket.rs, that will provide an example of how to create web applications in Rust.

## Helpful commands

Local DB:
docker run --name mysql --rm -e MYSQL_ALLOW_EMPTY_PASSWORD=true -p 3306:3306 -d mysql

Setup the orm:
DATABASE_URL=mysql://root@127.0.0.1:3306/rustacean_web diesel setup

Migration helpers:
diesel migration generate create_posts
DATABASE_URL=mysql://root@127.0.0.1:3306/rustacean_web diesel migration run
DATABASE_URL=mysql://root@127.0.0.1:3306/rustacean_web diesel migration redo

Set rustup nightly in repo:
rustup override set nightly
