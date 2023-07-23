use std::{fmt, ops};

pub type PosUnit = i16;

#[derive(PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy, Debug)]
pub struct Pos {
    pub x: PosUnit,
    pub y: PosUnit,
}

impl_op_ex!(+|p1: &Pos, p2: &Pos| -> Pos { Pos::new(p1.x + p2.x, p1.y + p2.y) });
impl_op_ex!(-|p1: &Pos, p2: &Pos| -> Pos { Pos::new(p1.x - p2.x, p1.y - p2.y) });
impl_op_ex!(-|p: &Pos| -> Pos { Pos::new(-p.x, -p.y) });

impl Pos {
    pub const fn new(x: PosUnit, y: PosUnit) -> Self {
        Pos { x, y }
    }

    pub fn from<X: TryInto<PosUnit>, Y: TryInto<PosUnit>>(x: X, y: Y) -> Result<Self, String> {
        let x = x.try_into().map_err(|_| "Error while converting x")?;
        let y = y.try_into().map_err(|_| "Error while converting y")?;
        Ok(Pos::new(x, y))
    }

    pub fn is_diagonal(&self) -> bool {
        self.x != 0 && self.y != 0
    }

    pub fn is_straight(&self) -> bool {
        !self.is_diagonal()
    }

    pub fn components(&self) -> [Self; 2] {
        [Pos::new(self.x, 0), Pos::new(0, self.y)]
    }

    pub fn orthogonal(&self) -> [Self; 2] {
        let v = Pos::new(self.y, self.x);
        [v, -v]
    }

    pub fn norm(&self) -> f64 {
        let x_squared = self.x.pow(2) as f64;
        let y_squared = self.y.pow(2) as f64;
        (x_squared + y_squared).sqrt()
    }

    pub fn sign(&self) -> Self {
        Pos::new(self.x.signum(), self.y.signum())
    }

    pub fn up(&self) -> Self {
        self + UP
    }

    pub fn down(&self) -> Self {
        self + DOWN
    }

    pub fn left(&self) -> Self {
        self + LEFT
    }

    pub fn right(&self) -> Self {
        self + RIGHT
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

pub const UP: Pos = Pos::new(0, -1);
pub const DOWN: Pos = Pos::new(0, 1);
pub const LEFT: Pos = Pos::new(-1, 0);
pub const RIGHT: Pos = Pos::new(1, 0);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
