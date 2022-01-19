# Hackerrank Meets AoC

---

## 1. How to run

### 1.1 Language

Download rust and cd into the root directory of the project. Make sure the
current directory is in the stable rust chain `rustup set stable`. Use
`cargo run` to run the project.

### 1.2 Database

Download Mysql server, and create your own user. in the root of your project,
create a file called `.env` and put a connection string in the following format:

`DATABASE_URL=mysql://username:password@localhost/database`

Where `username`, `password` and `database` are your sql names.

#### Download the MySQL C API

- Ubuntu Linux: `sudo apt install libmysqlclient-dev`

- Any other distribution/OS:
  - look in your distribution/OS official software repositories
  - or check the [MySQL official site](https://dev.mysql.com/downloads/c-api/)

### install diesel_cli (just with mysql)
- run the migrations `diesel migration run`
- and then you can run the site
