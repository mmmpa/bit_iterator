# bit_iterator

This iterates each bit of uint as a bool.

```rust
use crate::BitIterator;

#[test]
fn test8() {
    let txt = BitIterator::from(0b10010011 as u8)
        .fold(String::new(), |a, bit| format!("{}{}", a, bit as u8));
    assert_eq!("10010011", txt)
}

#[test]
fn test32() {
    let txt = BitIterator::from(0b10010011 as u32)
        .fold(String::new(), |a, bit| format!("{}{}", a, bit as u8));
    assert_eq!("00000000000000000000000010010011", txt)
}
```
