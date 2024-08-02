<h1 align="center">An analog of the decorator from Python 🐍</h1>

<h2 align="center">Usage example</h2>

```rust
use execute_macro::{execute};

#[tokio::main]
async fn main() {
    example().await;
}

#[execute(
    before="println!(\"Before\");",
    after="println!(\"After\");"
)]
async fn example() {
    println!("Executed inside");
}

```

<h2 align="center">Return</h2>

```
Before
Executed inside
After
```