use std::fmt::Display;

pub struct Bitboard {
    pub data: u64,
}

impl Bitboard {
    pub fn new() -> Self {
        Self { data: 0 }
    }

    pub fn from(data: u64) -> Self {
        Self { data }
    }

    pub fn set_bit(&mut self, bit: u8) {
        self.data |= 1 << bit;
    }

    pub fn get_bit(&self, bit: u8) -> bool {
        (self.data & (1 << bit)) != 0
    }

    pub fn clear_bit(&mut self, bit: u8) {
        self.data &= !(1 << bit);
    }

    pub fn shift_up(&mut self) {
        self.data <<= 8;
    }

    pub fn shift_down(&mut self) {
        self.data >>= 8;
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.data.to_be_bytes().iter().enumerate() {
            write!(f, "{:08b}", byte)?;
            if i < 7 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bitboard() {
        let bitboard = Bitboard::new();
        assert_eq!(bitboard.data, 0);
    }

    #[test]
    fn test_new_bitboard_display() {
        let bitboard = Bitboard::new();
        assert_eq!(format!("{}", bitboard), "00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000");
    }

    #[test]
    fn test_from_bitboard() {
        let bitboard = Bitboard::from(123456789);
        assert_eq!(bitboard.data, 123456789);
    }

    #[test]
    fn test_from_bitboard_display() {
        let bitboard = Bitboard::from(123456789);
        assert_eq!(format!("{}", bitboard), "00000000 00000000 00000000 00000000 00000111 01011011 11001101 00010101");
    }

    #[test]
    fn test_set_bit() {
        let mut bitboard = Bitboard::new();
        bitboard.set_bit(3);
        assert_eq!(bitboard.data, 0b1000);
    }

    #[test]
    fn test_get_bit() {
        let bitboard = Bitboard::from(0b1000);
        assert!(bitboard.get_bit(3));
        assert!(!bitboard.get_bit(2));
    }

    #[test]
    fn test_clear_bit() {
        let mut bitboard = Bitboard::from(0b1111);
        bitboard.clear_bit(2);
        assert_eq!(bitboard.data, 0b1011);
    }

    #[test]
    fn test_shift_up() {
        let mut bitboard = Bitboard::from(0b11111111);
        bitboard.shift_up();
        assert_eq!(bitboard.data, 0b1111111100000000);
    }

    #[test]
    fn test_shift_down() {
        let mut bitboard = Bitboard::from(0b1111111100000000);
        bitboard.shift_down();
        assert_eq!(bitboard.data, 0b11111111);
    }

    #[test]
    fn test_set_and_get_edge_cases() {
        let mut bitboard = Bitboard::new();

        bitboard.set_bit(0);
        assert!(bitboard.get_bit(0));

        bitboard.set_bit(63);
        assert!(bitboard.get_bit(63));

        assert!(!bitboard.get_bit(1));
        assert!(!bitboard.get_bit(62));
    }

    #[test]
    fn test_clear_edge_cases() {
        let mut bitboard = Bitboard::from(u64::MAX);

        bitboard.clear_bit(0);
        assert!(!bitboard.get_bit(0));

        bitboard.clear_bit(63);
        assert!(!bitboard.get_bit(63));
    }
}