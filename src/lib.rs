pub struct BitIterator<T> {
    n: T,
    i: u8,
}

macro_rules! define_into_bit_iterator {
    ( $( $type:tt => $initial:expr ),* $(,)? ) => { $(
        impl BitIterator<$type> {
            pub fn new(n: $type) -> Self {
                Self { n, i: $initial }
            }
        }

        impl From<$type> for BitIterator<$type> {
            fn from(n: $type) -> Self {
                Self::new(n)
            }
        }

        impl Iterator for BitIterator<$type> {
            type Item = bool;

            fn next(&mut self) -> Option<Self::Item> {
                if self.i == 0 {
                    return None;
                }

                self.i -= 1;
                Some(((self.n >> self.i) & 1) == 1)
            }
        }
    )* };
}

define_into_bit_iterator!(
    u8 => 8,
    u16 => 16,
    u32 => 32,
    u64 => 64,
    u128 => 128,
);

#[cfg(test)]
mod tests {
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
}
