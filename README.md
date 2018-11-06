# consul-rs

A consul client lib in Rust.(WIP)

## Usage

```rust
extern crate consul_rs;

use consul_rs::Client;

fn main() {
    let c = Client::new("localhost", 8500);
    let pairs = c.kv_get("test-key").unwrap();
    let pair = &pairs[0];
    let v = pair.get_value().unwrap();
    assert_eq!(b"hello"[..].to_vec(), v);
}
```
