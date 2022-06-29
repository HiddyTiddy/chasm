use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Registers {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
}

impl FromStr for Registers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Registers::*;
        match s {
            "R0" => Ok(R0),
            "R1" => Ok(R1),
            "R2" => Ok(R2),
            "R3" => Ok(R3),
            "R4" => Ok(R4),
            "R5" => Ok(R5),
            "R6" => Ok(R6),
            "R7" => Ok(R7),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_3::registers::Registers;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        // lol
        assert_eq!(Registers::from_str("R0"), Ok(Registers::R0));
        assert_eq!(Registers::from_str("R1"), Ok(Registers::R1));
        assert_eq!(Registers::from_str("R2"), Ok(Registers::R2));
        assert_eq!(Registers::from_str("R3"), Ok(Registers::R3));
        assert_eq!(Registers::from_str("R4"), Ok(Registers::R4));
        assert_eq!(Registers::from_str("R5"), Ok(Registers::R5));
        assert_eq!(Registers::from_str("R6"), Ok(Registers::R6));
        assert_eq!(Registers::from_str("R7"), Ok(Registers::R7));
        assert_eq!(Registers::from_str("R8"), Err(()));
    }
}
