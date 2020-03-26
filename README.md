# Arteria

Run and Share Perl source code on Web.  
The goal of this project is to create a Perl version of RunKit.

## Requirements

* Rust 1.42.0 or higher
* Node.js 10.x or higher
* Docker (Server API version 1.40)
* MySQL 8.x or higher

## Environment Variables

| Variable Name           | Example Value | Description                       |
| ----------------------- | ------------- | --------------------------------- |
| `ARTERIA_DATABASE_HOST` | `localhost`   | MySQL database connection address |
| `ARTERIA_DATABASE_USER` | `root`        | MySQL database user               |
| `ARTERIA_DATABASE_PASS` | ``            | MySQL database password           |
| `ARTERIA_DATABASE_PORT` | `3306`        | MySQL database connection port    |
| `ARTERIA_HASH_SALT`     | `Ex@mp1e`     | Hash ID salt value                |
| `ARTERIA_SERVER_BIND`   | `127.0.0.1`   | Arteria server binding address    |
| `ARTERIA_SERVER_PORT`   | `3000`        | Arteria server binding port       |

## FAQ

### What is different from other web services?

- You can write code in Visual Studio Code.
- You can use CPAN modules.
- You can host Arteria yourself.
- Account registration is not required.
- The source code is public (OSS).

### What is different from Altar?

Altar has the following problems:

- difficult to create an own environment
- be slow to get a result

Arteria tries to solve these problems.

### Why Rust instead of Perl?

This is a personal hobby project.  
I was interested in writing Web application on Rust.

## License

This software is licensed under the AGPL-3.0.
