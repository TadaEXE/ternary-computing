use super::trit::Trit;

pub trait TernaryArithmetic {
    fn add(&mut self, other: &Trit) -> Trit;
    fn sub(&mut self, other: &Trit) -> Trit;
    fn mul(&mut self, other: &Trit) -> Trit;
    fn div(&mut self, other: &Trit) -> Trit;

    // fn imut_add(&self, other: &Trit) -> (Trit, Trit);
}

pub trait TernaryLogic {
    fn and(&mut self, other: &Trit) -> ();
    fn or(&mut self, other: &Trit) -> ();
    fn xor(&mut self, other: &Trit) -> ();
    fn not(&mut self) -> ();
}
