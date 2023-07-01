# shuttle-seaorm

A `seaorm` resource for Shuttle that returns a SeaORM connection pool.

## Usage

Simply load up a Shuttle project and replace `shuttle_shared_db` with `shuttle_seaorm` like so:

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
