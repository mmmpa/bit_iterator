[![CircleCI](https://circleci.com/gh/mmmpa/bit_iterator.svg?style=shield)](https://circleci.com/gh/mmmpa/bit_iterator)

# bit_iterator

This iterates each bit of uint as a bool.

This is `no_std`.

```rust
use bit_iterator::BitIterator;

#[test]
fn test8() {
    let txt =
        BitIterator::from(0b10010011 as u8)
            .enumerate()
            .fold([0; 8], |mut a, (i, bit)| {
                a[i] = bit as u8;
                a
            });
    assert_eq!([1, 0, 0, 1, 0, 0, 1, 1], txt)
}

#[test]
fn test32() {
    let txt =
        BitIterator::from(0b10010011 as u32)
            .enumerate()
            .fold([0; 32], |mut a, (i, bit)| {
                a[i] = bit as u8;
                a
            });
    assert_eq!(
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 1, 1
        ],
        txt
    )
}
```
