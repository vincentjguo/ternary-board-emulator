#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Trit {
    N = -1,
    #[default]
    Z = 0,
    P = 1,
}

impl Trit {
    pub fn state(value: i8) -> Self {
        match value {
            -1 => Trit::N,
            0 => Trit::Z,
            1 => Trit::P,
            e => panic!("Invalid state value {}", e),
        }
    }
    pub fn value(&self) -> i8 {
        match self {
            Trit::N => -1,
            Trit::Z => 0,
            Trit::P => 1,
        }
    }
    pub fn set_state(&mut self, other: &Trit) {
        *self = Self::state(other.value());
    }
}


impl std::fmt::Debug for Trit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trit::N => write!(f, "-"),
            Trit::Z => write!(f, "0"),
            Trit::P => write!(f, "+"),
        }
    }
}
