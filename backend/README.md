# Arteria

Arteria backend server written in Rust.

## Environment Variables

| Variable Name                              | Example Value           | Description                       |
| ------------------------------------------ | ----------------------- | --------------------------------- |
| `ARTERIA_CORS_ALLOWED_HOST`                | `http://localhost:8080` | CORS allowed domains              |
| `ARTERIA_CONTAINER_CONCURRENCY`            | `16`                    |                                   |
| `ARTERIA_CONTAINER_EXECUTOR_CPU_LIMIT`     | `(none)`                |                                   |
| `ARTERIA_CONTAINER_EXECUTOR_MEMORY_LIMIT`  | `(none)`                |                                   |
| `ARTERIA_CONTAINER_EXECUTOR_RUNTIME`       | `runsc`                 |                                   |
| `ARTERIA_CONTAINER_EXECUTOR_TIMEOUT`       | `10`                    |                                   |
| `ARTERIA_CONTAINER_INSTALLER_CPU_LIMIT`    | `(none)`                |                                   |
| `ARTERIA_CONTAINER_INSTALLER_MEMORY_LIMIT` | `(none)`                |                                   |
| `ARTERIA_DATABASE_HOST`                    | `localhost`             | MySQL database connection address |
| `ARTERIA_DATABASE_USER`                    | `root`                  | MySQL database user               |
| `ARTERIA_DATABASE_PASS`                    | `(none)`                | MySQL database password           |
| `ARTERIA_DATABASE_PORT`                    | `3306`                  | MySQL database connection port    |
| `ARTERIA_HASH_SALT`                        | `Ex@mp1e`               | Hash ID salt value                |
| `ARTERIA_SERVER_BIND`                      | `127.0.0.1`             | Arteria server binding address    |
| `ARTERIA_SERVER_PORT`                      | `3000`                  | Arteria server binding port       |
