use super::ternary_traits::{TernaryArithmetic, TernaryLogic};
use core::panic;
use std::convert::{From, TryFrom};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TernaryValue {
    ZERO = 0,

    ONE = 1,
    TWO = 2,

    MINUS = -1,
}

impl TernaryValue {
    pub const PLUS: TernaryValue = TernaryValue::ONE;

    pub fn new() -> Self {
        TernaryValue::ZERO
    }
}

impl From<TernaryValue> for i8 {
    fn from(tval: TernaryValue) -> Self {
        match tval {
            TernaryValue::ZERO => 0,
            TernaryValue::ONE => 1,
            TernaryValue::TWO => 2,
            TernaryValue::MINUS => -1,
        }
    }
}

impl TryFrom<i8> for TernaryValue {
    type Error = &'static str;

    fn try_from(val: i8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(TernaryValue::ZERO),
            1 => Ok(TernaryValue::ONE),
            2 => Ok(TernaryValue::TWO),
            -1 => Ok(TernaryValue::MINUS),
            _ => Err("Value that can't be converted to ternary value was given."),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Trit {
    value: TernaryValue,
}

impl Trit {
    pub fn new(value: TernaryValue) -> Self {
        Trit { value }
    }

    pub fn flip(&mut self) -> () {
        let new_value: i8 = self.value.into();
        self.value = TernaryValue::try_from(new_value * -1).unwrap();
    }
}

//==BalancedTrit===================================================================================
pub struct BalancedTrit {
    trit: Trit,
}

impl TernaryArithmetic for BalancedTrit {
    fn add(&mut self, other: &Trit) -> Trit {
        let lhs: i8 = self.trit.value.into();
        let rhs: i8 = other.value.into();
        let sum = lhs + rhs;
        if sum > 1 {
            self.trit.value = TernaryValue::MINUS;
            Trit::new(TernaryValue::PLUS)
        } else if sum < -1 {
            self.trit.value = TernaryValue::PLUS;
            Trit::new(TernaryValue::MINUS)
        } else {
            self.trit.value = TernaryValue::try_from(sum).unwrap();
            Trit::new(TernaryValue::ZERO)
        }
    }

    fn sub(&mut self, other: &Trit) -> Trit {
        let mut neg_other = other.clone();
        neg_other.flip();
        self.add(&neg_other)
    }

    fn mul(&mut self, other: &Trit) -> Trit {
        let lhs: i8 = self.trit.value.into();
        let rhs: i8 = other.value.into();
        self.trit.value = TernaryValue::try_from(lhs * rhs).unwrap();
        Trit::new(TernaryValue::ZERO)
    }

    fn div(&mut self, other: &Trit) -> Trit {
        let lhs: i8 = self.trit.value.into();
        let rhs: i8 = other.value.into();
        if rhs == 0 {
            panic!("Divide by zero");
        }
        self.trit.value = TernaryValue::try_from(lhs / rhs).unwrap();
        Trit::new(TernaryValue::ZERO)
    }
}

impl TernaryLogic for BalancedTrit {
    fn and(&mut self, other: &Trit) -> () {
        let lhs = self.trit.value;
        let rhs = other.value;

        if lhs == TernaryValue::MINUS || rhs == TernaryValue::MINUS {
            self.trit.value = TernaryValue::MINUS;
        } else if lhs == TernaryValue::ZERO || rhs == TernaryValue::ZERO {
            self.trit.value = TernaryValue::ZERO;
        } else {
            self.trit.value = TernaryValue::PLUS;
        }
    }

    fn or(&mut self, other: &Trit) -> () {
        let lhs = self.trit.value;
        let rhs = other.value;

        if lhs == TernaryValue::PLUS || rhs == TernaryValue::PLUS {
            self.trit.value = TernaryValue::PLUS;
        } else if lhs == TernaryValue::ZERO || rhs == TernaryValue::ZERO {
            self.trit.value = TernaryValue::ZERO;
        } else {
            self.trit.value = TernaryValue::MINUS;
        }
    }

    fn xor(&mut self, other: &Trit) -> () {
        let lhs = self.trit.value;
        let rhs = other.value;

        if lhs == TernaryValue::ZERO || rhs == TernaryValue::ZERO {
            self.trit.value = TernaryValue::ZERO;
        } else if lhs == rhs {
            self.trit.value = TernaryValue::MINUS;
        } else {
            self.trit.value = TernaryValue::PLUS;
        }
    }

    fn not(&mut self) -> () {
        self.trit.flip();
    }
}

//==UnbalancedTrit=================================================================================

struct UnbalancedTrit {
    trit: Trit,
}

impl TernaryArithmetic for UnbalancedTrit {
    fn add(&mut self, other: &Trit) -> Trit {
        
    }

    fn sub(&mut self, other: &Trit) -> Trit {
        
    }

    fn mul(&mut self, other: &Trit) -> Trit {
        
    }

    fn div(&mut self, other: &Trit) -> Trit {
        
    }
}

























