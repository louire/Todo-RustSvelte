# TODO RUST🦀SVELTE

The web application being developed uses the Rust and SvelteKit stack.

Tokio is a Rust library designed for building scalable and high-performance network applications. Tokio employs an asynchronous programming model to efficiently handle multiple network connections simultaneously, making it an ideal choice for web applications that need to manage numerous requests efficiently.

On the other hand, SvelteKit is a JavaScript framework for building web applications. SvelteKit leverages compile-time technology, meaning that applications are compiled before being sent to the browser, resulting in faster and more secure performance. Additionally, SvelteKit offers a wealth of useful features for web application development, such as routing, form handling, and API support.

In summary, the combination of Rust and Tokio with SvelteKit is an excellent choice for building scalable, secure, and high-performance web applications.


Install `sqlx-cli` using `cargo install sqlx-cli`, then run following:

```
git clone https://github.com/knarkzel/todo
cd todo/
```

## backend

```
cd backend/
sqlx database setup
cargo run
```

## frontend

```
cd frontend/
npm install
npm run dev
```
