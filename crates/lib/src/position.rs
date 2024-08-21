use std::ops::Index;

use glam::I16Vec2;

pub type Pos = I16Vec2;
pub type PosUnit = <Pos as Index<usize>>::Output;

pub trait MyFuncs: Sized {
    fn convert<T: Into<PosUnit>>(x: T, y: T) -> Self;

    fn try_convert<T: TryInto<PosUnit>>(x: T, y: T) -> Result<Self, T::Error>;

    fn is_diagonal(&self) -> bool;

    fn components(&self) -> [Self; 2];

    fn orthogonal(&self) -> [Self; 2];

    fn norm(&self) -> f64;

    fn up(&self) -> Self;

    fn down(&self) -> Self;

    fn left(&self) -> Self;

    fn right(&self) -> Self;

    fn up_left(&self) -> Self {
        self.up().left()
    }

    fn up_right(&self) -> Self {
        self.up().right()
    }

    fn down_left(&self) -> Self {
        self.down().left()
    }

    fn down_right(&self) -> Self {
        self.down().right()
    }
}

impl MyFuncs for Pos {
    fn convert<T: Into<PosUnit>>(x: T, y: T) -> Self {
        let x = x.into();
        let y = y.into();
        Self::new(x, y)
    }

    fn try_convert<T: TryInto<PosUnit>>(x: T, y: T) -> Result<Self, T::Error> {
        let x = x.try_into()?;
        let y = y.try_into()?;
        Ok(Self::new(x, y))
    }

    fn is_diagonal(&self) -> bool {
        self.x != 0 && self.y != 0
    }

    fn components(&self) -> [Self; 2] {
        [self.with_y(0), self.with_x(0)]
    }

    fn orthogonal(&self) -> [Self; 2] {
        let v = self.perp();
        [v, -v]
    }

    fn norm(&self) -> f64 {
        (self.length_squared() as f64).sqrt()
    }

    fn up(&self) -> Self {
        *self + Pos::NEG_Y
    }

    fn down(&self) -> Self {
        *self + Pos::Y
    }

    fn left(&self) -> Self {
        *self + Pos::NEG_X
    }

    fn right(&self) -> Self {
        *self + Pos::X
    }
}
