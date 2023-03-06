# space-lib

```
cargo add space-lib
```

This crate provides WebAssembly host functions and other utilities
for Space Operator.

## Macro

```rust
use space_lib::space;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Input {
    value: usize,
    name: String,
}

#[derive(Serialize)]
struct Output {
    value: usize,
    name: String,
}

#[space]
fn main(input: Input) -> Output {
    Output {
        value: input.value * 2,
        name: input.name.chars().rev().collect(),
    }
}
```

## Result

```rust
use space_lib::{space, Result};

#[space]
fn main() -> Result<u64> {
    Ok("123".parse()?)
}
```

## HTTP client

```rust
use space_lib::Request;

let body = Request::get("https://www.spaceoperator.com")
    .call()?
    .into_string()?;
```

## Supabase

```rust
use space_lib::Supabase;

let client = Supabase::new("https://hyjbiblkjrrvkzaqsyxe.supabase.co")
    .insert_header("apikey", "anon_api_key");

let rows = client
    .from("dogs")
    .select("name")
    .execute()?
    .into_string()?;
```

## Solana

```rust
use space_lib::Solana;

let client = Solana::new("https://api.devnet.solana.com");
let balance = client.get_balance("base58_encoded_pubkey")?;
```

