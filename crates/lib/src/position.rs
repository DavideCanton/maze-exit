use anyhow;
use glam::I64Vec2;

pub type PosUnit = i64;
pub type Pos = I64Vec2;

pub trait MyFuncs: Sized {
    fn convert<X: TryInto<PosUnit>, Y: TryInto<PosUnit>>(x: X, y: Y) -> anyhow::Result<Self>;

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
    fn convert<X: TryInto<PosUnit>, Y: TryInto<PosUnit>>(x: X, y: Y) -> anyhow::Result<Self> {
        let x = x.try_into().unwrap_or(0); // .map_err(|e| anyhow::anyhow!(e))?;
        let y = y.try_into().unwrap_or(0); //map_err(|e| anyhow::anyhow!(e))?;
        Ok(Pos::new(x, y))
    }

    fn is_diagonal(&self) -> bool {
        *self != Pos::ZERO
    }

    fn components(&self) -> [Self; 2] {
        [self.with_x(0), self.with_y(0)]
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
