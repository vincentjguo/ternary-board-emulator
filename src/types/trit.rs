use std::cmp::Ordering;

pub(crate) enum Trit {
    N = -1,
    Z = 0,
    P = 1
}

impl Trit {
    pub fn state(value: i8) -> Self {
        match value {
            -1 => Trit::N,
            0 => Trit::Z,
            1 => Trit::P,
            e => panic!("Invalid state value {}", e)
        }
    }
}

impl std::fmt::Debug for Trit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trit::N => write!(f, "-"),
            Trit::Z => write!(f, "0"),
            Trit::P => write!(f, "+")
        }
    }
}


impl PartialOrd for Trit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}
