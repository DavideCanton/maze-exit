use std::{fmt, ops};

#[derive(PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl_op_ex!(+|p1: &Pos, p2: &Pos| -> Pos { Pos::new(p1.x + p2.x, p1.y + p2.y) });
impl_op_ex!(-|p1: &Pos, p2: &Pos| -> Pos { Pos::new(p1.x - p2.x, p1.y - p2.y) });
impl_op_ex!(-|p: &Pos| -> Pos { Pos::new(-p.x, -p.y) });

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
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
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    pub fn sign(&self) -> Self {
        Pos::new(self.x.signum(), self.y.signum())
    }

    pub fn move_up(&self) -> Self {
        self + Pos::new(0, -1)
    }

    pub fn move_down(&self) -> Self {
        self + Pos::new(0, 1)
    }

    pub fn move_left(&self) -> Self {
        self + Pos::new(-1, 0)
    }

    pub fn move_right(&self) -> Self {
        self + Pos::new(1, 0)
    }

    pub fn move_up_left(&self) -> Self {
        self.move_up().move_left()
    }

    pub fn move_up_right(&self) -> Self {
        self.move_up().move_right()
    }

    pub fn move_down_left(&self) -> Self {
        self.move_down().move_left()
    }

    pub fn move_down_right(&self) -> Self {
        self.move_down().move_right()
    }
}

impl From<(i32, i32)> for Pos {
    fn from(t: (i32, i32)) -> Self {
        Pos::new(t.0, t.1)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
