use std::fmt::write;

use rand::{rngs::ThreadRng, seq::IteratorRandom};

#[derive(Eq, PartialEq, Clone, Copy, Default, Debug)]
pub enum Turn {
    #[default]
    U,
    L, F, R, B, D
}

impl Turn {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Turn::U),
            'D' => Some(Turn::D),
            'F' => Some(Turn::F),
            'B' => Some(Turn::B),
            'L' => Some(Turn::L),
            'R' => Some(Turn::R),
            _ => None,
        }
    }
    fn is_opposite(&self, other: Turn) -> bool {
        use Turn::*;
        match (self, other) {
            (U, D) | (D, U) | (L, R) | (R, L) | (F, B) | (B, F) => true,
            _ => false
        }
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Turn::U => "U",
            Turn::D => "D",
            Turn::F => "F",
            Turn::B => "B",
            Turn::L => "L",
            Turn::R => "R",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum TurnDir {
    #[default]
    None,
    One, Two, Prime
}

impl TurnDir {
    fn as_u8(self) -> u8 {
        match self {
            TurnDir::None => 0,
            TurnDir::One => 1,
            TurnDir::Two => 2,
            TurnDir::Prime => 3,
        }
    }

    fn from_u8(v: u8) -> Self {
        match v % 4 {
            0 => TurnDir::None,
            1 => TurnDir::One,
            2 => TurnDir::Two,
            _ => TurnDir::Prime,
        }
    }
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(TurnDir::None),
            ' ' |'1' => Some(TurnDir::One),
            '2' => Some(TurnDir::Two),
            '\'' | '3' => Some(TurnDir::Prime),
            _ => None,
        }
    }
}

impl std::ops::Add for TurnDir {
    type Output = TurnDir;

    fn add(self, rhs: TurnDir) -> TurnDir {
        TurnDir::from_u8(self.as_u8() + rhs.as_u8())
    }
}

impl std::fmt::Display for TurnDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TurnDir::None => "0",
            TurnDir::One => "",
            TurnDir::Two => "2",
            TurnDir::Prime => "'",
        })
    }
}

// Struct for different move types, includes buth which face is turned and the direction
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Twist {
    pub turn: Turn,
    pub dir: TurnDir,
}

impl Twist {
    pub const fn new(turn: Turn, dir: TurnDir) -> Self {
        Self { turn, dir }
    }

    pub fn new_random(rng: &mut ThreadRng, prev_turn: Option<Turn>) -> Self {
        Self::allowed_moves(prev_turn).choose(rng).unwrap()
    }

    const fn const_default() -> Self {
        Self { turn: Turn::U, dir: TurnDir::None }
    }

    pub fn inverse(self) -> Self {
        match self {
            Twist { turn, dir: TurnDir::None } => Twist { turn, dir: TurnDir::None },
            Twist { turn, dir: TurnDir::One } => Twist { turn, dir: TurnDir::Prime },
            Twist { turn, dir: TurnDir::Two } => Twist { turn, dir: TurnDir::Two },
            Twist { turn, dir: TurnDir::Prime } => Twist { turn, dir: TurnDir::One },
        }
    }

    pub fn try_add(self, other: Twist) -> Option<Twist> {
        if self.turn != other.turn { return None; }
        
        Some(Twist { turn: self.turn, dir: self.dir + other.dir })
    }

    pub const ALL_TWISTS: [Twist; 18] = [
        Twist { turn: Turn::U, dir: TurnDir::One },
        Twist { turn: Turn::U, dir: TurnDir::Two },
        Twist { turn: Turn::U, dir: TurnDir::Prime },
        Twist { turn: Turn::D, dir: TurnDir::One },
        Twist { turn: Turn::D, dir: TurnDir::Two },
        Twist { turn: Turn::D, dir: TurnDir::Prime },
        Twist { turn: Turn::F, dir: TurnDir::One },
        Twist { turn: Turn::F, dir: TurnDir::Two },
        Twist { turn: Turn::F, dir: TurnDir::Prime },
        Twist { turn: Turn::B, dir: TurnDir::One },
        Twist { turn: Turn::B, dir: TurnDir::Two },
        Twist { turn: Turn::B, dir: TurnDir::Prime },
        Twist { turn: Turn::L, dir: TurnDir::One },
        Twist { turn: Turn::L, dir: TurnDir::Two },
        Twist { turn: Turn::L, dir: TurnDir::Prime },
        Twist { turn: Turn::R, dir: TurnDir::One },
        Twist { turn: Turn::R, dir: TurnDir::Two },
        Twist { turn: Turn::R, dir: TurnDir::Prime },
    ];

    pub fn allowed_moves(prev: Option<Turn>) -> impl Iterator<Item = Twist> {
        Self::allowed_moves_from_moveset(&Self::ALL_TWISTS, prev)
    }
    pub fn allowed_moves_from_moveset(moveset: &[Twist], prev: Option<Turn>) -> impl Iterator<Item = Twist> {
        moveset.iter().filter(move |m| {
            match prev {
                None => true,
                Some(p) => match p {
                    Turn::U | Turn::R | Turn::F => m.turn != p,
                    Turn::L => {m.turn != Turn::L && m.turn != Turn::R}
                    Turn::B => {m.turn != Turn::B && m.turn != Turn::F}
                    Turn::D => {m.turn != Turn::D && m.turn != Turn::U}
                }
            }
        }).cloned()
    }
}

impl std::fmt::Display for Twist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.turn, self.dir)
    }
}

pub struct ConstAlgorithm<const N: usize> {
    pub twists: [Twist; N],
}

impl<const N: usize> ConstAlgorithm<N> {
    pub const SUPERFLIP: ConstAlgorithm<20> = ConstAlgorithm::from_str("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2");
    pub const J_PERM: ConstAlgorithm<14> = ConstAlgorithm::from_str("R U R' F' R U R' U' R' F R2 U' R' U'");
    pub const T_PERM: ConstAlgorithm<14> = ConstAlgorithm::from_str("R U R' U' R' F R2 U' R' U' R U R' F'");
    pub const UA_PERM: ConstAlgorithm<11> = ConstAlgorithm::from_str("R U' R U R U R U' R' U' R2");

    pub const fn from_str(s: &str) -> Self {
        let mut twists = [Twist::const_default(); N];
        let mut len = 0;
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            if c != ' ' {
                if let Some(t) = Turn::from_char(c) {
                    twists[len] = Twist::new(t, TurnDir::One);
                    len += 1;
                } else if let Some(d) = TurnDir::from_char(c) {
                    if len > 0 {
                        twists[len - 1].dir = d;
                    }
                }
            }
            i += 1;
        }
        assert!(len == N);
        Self { twists }
    }
    pub fn to_algorithm(&self) -> Algorithm {
        Algorithm { twists: self.twists.to_vec() }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Algorithm {
    pub twists: Vec<Twist>,
}

impl Algorithm {
    pub fn new(twists: Vec<Twist>) -> Self {
        Self { twists }
    }

    pub fn new_random(rng: &mut ThreadRng, length: usize) -> Self {
        let mut twists = Vec::with_capacity(length);
        let mut prev_turn = None;
        for _ in 0..length {
            let twist = Twist::new_random(rng, prev_turn);
            twists.push(twist);
            prev_turn = Some(twist.turn)
        }
        Self { twists }
    }

    // Creates algorithm from standard cube notation
    pub fn from_str(str: &str) -> Self {
        let mut twists = Vec::new();
        for c in str.chars() {
            if c.is_whitespace() { continue; }
            if let Some(t) = Turn::from_char(c) {
                twists.push(Twist::new(t, TurnDir::One));
            }
            else if let Some(d) = TurnDir::from_char(c) {
                if let Some(last) = twists.last_mut() {
                    last.dir = d;
                }
            }
        }
        Self { twists }
    }
    pub fn append(&mut self, other: &mut Self) {
        self.twists.append(&mut other.twists);
    }

    // Collects twists together to shorten algs. If two last moves are opposites, then they do not influence each other and both of these are compared to the twist checked
    // Removes uneccesary moves with TurnDir::None
    pub fn simplify(&mut self) {
        let mut simplified: Vec<Twist> = Vec::new();
        for twist in &self.twists {
            if twist.dir == TurnDir::None {
                continue;
            }
            let mut push_twist = true;
            let len = simplified.len();
            if let Some(last) = simplified.last_mut() {
                let last_turn = last.turn; // Defined here becasue last.turn can not be referenced later when needed because of rusts borrowing rules

                if let Some(added) = last.try_add(*twist) {
                    if added.dir == TurnDir::None {
                        simplified.pop();
                    }
                    else {
                        *last = added;
                    }
                    push_twist = false;
                }
                else if len >= 2 {
                    let second_last = &mut simplified[len - 2];
                    if second_last.turn.is_opposite(last_turn) {
                        if let Some(added) = second_last.try_add(*twist) {
                            if added.dir == TurnDir::None {
                                simplified.remove(len - 2);
                            }
                            else {
                                *second_last = added;
                            }
                            push_twist = false;
                        }
                    }
                }
            }
            if push_twist {
                simplified.push(*twist);
            }
        }
        self.twists = simplified;
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.twists {
            write!(f, "{} ", m)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_and_non_const_alg_from_string_same_result() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let original = Algorithm::new_random(&mut rng, 50);
            let fmt = format!("{}", original);

            let non_const = Algorithm::from_str(&fmt);
            let cons: ConstAlgorithm<50> = ConstAlgorithm::from_str(&fmt);

            assert_eq!(original, non_const);
            assert_eq!(original, cons.to_algorithm());
        }
    }

    #[test]
    fn alg_simplify() {
        let mut alg = Algorithm::from_str("R R R R");
        alg.simplify();
        assert_eq!(alg, Algorithm::new(vec![]));

        let mut alg = Algorithm::from_str("R L R");
        alg.simplify();
        assert_eq!(alg, Algorithm::from_str("R2 L"));

        let mut alg = Algorithm::from_str("L R R");
        alg.simplify();
        assert_eq!(alg, Algorithm::from_str("L R2"));

        let mut alg = Algorithm::from_str("L F L");
        alg.simplify();
        assert_eq!(alg, Algorithm::from_str("L F L"));
    }
}