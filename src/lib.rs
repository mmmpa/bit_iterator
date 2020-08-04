pub enum Bit {
    Zero = 0,
    One = 1,
}

macro_rules! define_bit_iterator {
    ( $($name:tt, $type:tt, $initial:tt, ),* $(,)? ) => { $(

    )* };
}

pub struct BitIterator8 {
    n: u8,
    i: u8,
}

impl From<u8> for BitIterator8 {
    fn from(n: u8) -> Self {
        Self { n, i: 8 }
    }
}

impl Iterator for BitIterator8 {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            return None;
        }
        self.i -= 1;
        Some(match (self.n >> self.i) & 1 {
            1 => Bit::One,
            _ => Bit::Zero,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::BitIterator8;
    #[test]
    fn test8() {
        let txt = BitIterator8::from(0b10010010)
            .fold(String::new(), |a, bit| a + &(bit as u8).to_string());
        assert_eq!("10010010", txt)
    }
}
