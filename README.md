# Modern Web Booth Question

## How to start

1. [Install Rust](https://www.rust-lang.org/tools/install)

```sh
$ curl https://sh.rustup.rs -sSf | sh
```

2. Restart your terminal

3. Install wasm target

```sh
$ rustup target add wasm32-unknown-unknown
```

4. Install `wasm-pack`

```sh
$ cargo install wasm-pack
```

5. Clone the repo

```sh
$ git clone https://github.com/lili668668/modernweb-booth-question.git
```

6. Go to the project and build it

```sh
$ cd modernweb-booth-question

$ git checkout wasm

$ yarn serve # or npm run serve
```

7. Open your browser & visit [http://localhost:8080](http://localhost:8080)

## About us

We are Rust Taiwan. There is our chat group: [http://t.me/rust\_tw](http://t.me/rust_tw). Welcome to join us!
