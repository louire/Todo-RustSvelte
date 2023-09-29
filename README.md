# WORKING ON A BETTER README FILE, JUST FINISHED


# Todo-RustSvelte
crazy fkng god mode

first time using sveltekit but im using a template and a tailwind template soo YOLO . . .  yes im from the 2000


Rust: 
[dependencies]
    axum = { version = "0.6.20", features = ["form"] }
    axum-error = "0.2.0"
    dotenv = "0.15.0"
    serde = { version = "1.0.188", features = ["derive"] }
    sqlx = { version = "0.7.2", features = ["runtime-tokio", "tls-rustls", "sqlite"] }
    tokio = { version = "1.32.0", features = ["full"] }
    tower-http = { version = "0.4.4", features = ["cors"] }
Doing the first migration with sqlx and i love this message that showed to me
    "
Congratulations on creating your first migration!

Did you know you can embed your migrations in your application binary?
On startup, after creating your database connection or pool, add:

sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

Note that the compiler won't pick up new migrations if no Rust source files have changed.
You can create a Cargo build script to work around this with `sqlx migrate build-script`.

See: https://docs.rs/sqlx/0.5/sqlx/macro.migrate.html
"
