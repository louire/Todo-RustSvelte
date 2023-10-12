# TODO RUST🦀SVELTE

The web application being developed uses the Rust and SvelteKit stack.
<div>
  

<img src="https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white" height="35" alt="svelte logo"  />
  <img width="10" />
  <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge" height="35" alt="rust logo"  />
  <img width="10" />
  <img src="https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white" height="35" alt="sqlite logo"  />
  <img width="10" />
  <img src="https://img.shields.io/badge/Tokio-372213?style=for-the-badge&logo=tokio&logoColor=white" height="35" alt="tokio logo"  />
  <img width="10" />
  
</div>
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
