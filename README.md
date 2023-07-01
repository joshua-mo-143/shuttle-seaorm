# shuttle-seaorm

A `seaorm` resource for Shuttle that returns a SeaORM connection pool.

## Usage

You'll need to install seaORM on your Shuttle project to be able to use this. You can do that by running the following command:

```sh
cargo add sea-orm --features seaorm/runtime-tokio-native-tls,seaorm/sqlx-postgres
cargo add shuttle-seaorm --git https://github.com/joshua-mo-143/shuttle-seaorm.git
```

Then when you're writing your Shuttle app, annotate your main entrypoint function with `shuttle_seaorm` like so:

```rust
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

struct AppState {
    pool: DatabaseConnection,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_seaorm::Postgres] pool: DatabaseConnection) -> shuttle_axum::ShuttleAxum {
    let state = AppState { pool };
    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
```

Now you're ready to start using SeaORM!

## License

`shuttle-seaorm` is licensed under the [MIT license](http://opensource.org/licenses/MIT).

### Contribution

Contributions are welcome! Feel free to open a PR or issue.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, 
shall be licensed as above, without any additional terms or conditions.
