use std::{
    fmt,
    ops::{Add, Neg, Sub},
};

pub type PosUnit = i64;

#[derive(PartialEq, Default, Eq, Hash, Clone, Copy, Debug)]
pub struct Pos {
    pub x: PosUnit,
    pub y: PosUnit,
}

impl Pos {
    pub const fn new(x: PosUnit, y: PosUnit) -> Self {
        Pos { x, y }
    }

    pub const fn is_diagonal(&self) -> bool {
        self.x != 0 && self.y != 0
    }

    pub const fn is_straight(&self) -> bool {
        !self.is_diagonal()
    }

    pub fn components(&self) -> [Self; 2] {
        [Pos::new(self.x, 0), Pos::new(0, self.y)]
    }

    pub fn orthogonal(&self) -> [Self; 2] {
        let v = Pos::new(self.y, self.x);
        [v, -v]
    }

    #[inline(always)]
    pub fn norm(&self) -> f64 {
        ((self.x.pow(2) as f64) + (self.y.pow(2) as f64)).sqrt()
    }

    pub fn sign(&self) -> Self {
        Pos::new(self.x.signum(), self.y.signum())
    }

    pub fn up(&self) -> Self {
        *self + UP
    }

    pub fn down(&self) -> Self {
        *self + DOWN
    }

    pub fn left(&self) -> Self {
        *self + LEFT
    }

    pub fn right(&self) -> Self {
        *self + RIGHT
    }

    pub fn up_left(&self) -> Self {
        self.up().left()
    }

    pub fn up_right(&self) -> Self {
        self.up().right()
    }

    pub fn down_left(&self) -> Self {
        self.down().left()
    }

    pub fn down_right(&self) -> Self {
        self.down().right()
    }
}

const UP: Pos = Pos::new(0, -1);
const DOWN: Pos = Pos::new(0, 1);
const LEFT: Pos = Pos::new(-1, 0);
const RIGHT: Pos = Pos::new(1, 0);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<X: Into<PosUnit>, Y: Into<PosUnit>> From<(X, Y)> for Pos {
    fn from((x, y): (X, Y)) -> Self {
        Pos::new(x.into(), y.into())
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Pos::new(self.x - other.x, self.y - other.y)
    }
}

impl Neg for Pos {
    type Output = Self;

    fn neg(self) -> Self {
        Pos::new(-self.x, -self.y)
    }
}
