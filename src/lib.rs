pub enum Bit {
    Zero = 0,
    One = 1,
}

pub enum Uint {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub struct BitIterator {
    n: Uint,
    i: u8,
}

impl Iterator for BitIterator {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            return None;
        }
        self.i -= 1;
        let stand = match self.n {
            Uint::U8(n) => ((n >> self.i) & 1) == 1,
            Uint::U16(n) => ((n >> self.i) & 1) == 1,
            Uint::U32(n) => ((n >> self.i) & 1) == 1,
            Uint::U64(n) => ((n >> self.i) & 1) == 1,
            Uint::U128(n) => ((n >> self.i) & 1) == 1,
        };
        Some(match stand {
            true => Bit::One,
            false => Bit::Zero,
        })
    }
}

macro_rules! define_into_bit_iterator {
    ( $( $type:tt => { $en:expr, $initial:expr } ),* $(,)? ) => { $(
        impl From<$type> for BitIterator {
            fn from(n: $type) -> Self {
                Self {
                    n: $en(n),
                    i: $initial,
                }
            }
        }
    )* };
}

define_into_bit_iterator!(
    u8 => { Uint::U8, 8 },
    u16 => { Uint::U16, 16 },
    u32 => { Uint::U32, 32 },
    u64 => { Uint::U64, 64 },
    u128 => { Uint::U128, 128 },
);

#[cfg(test)]
mod tests {
    use crate::BitIterator;

    #[test]
    fn test8() {
        let txt = BitIterator::from(0b10010011 as u8)
            .fold(String::new(), |a, bit| a + &(bit as u8).to_string());
        assert_eq!("10010011", txt)
    }

    #[test]
    fn test32() {
        let txt = BitIterator::from(0b10010011 as u32)
            .fold(String::new(), |a, bit| a + &(bit as u8).to_string());
        assert_eq!("00000000000000000000000010010011", txt)
    }
}
