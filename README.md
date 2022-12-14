# space-lib

```
cargo add space-lib
```

This crate provides WebAssembly host functions for Space Operator.

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
