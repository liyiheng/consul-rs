extern crate consul_rs;

use consul_rs::Client;

#[test]
fn kv_get() {
    let c = Client::new("localhost", 8500);
    let pairs = c.kv_get("test-key").unwrap();
    let pair = &pairs[0];
    let v = pair.get_value().unwrap();
    assert_eq!(b"hello"[..].to_vec(), v);
}
