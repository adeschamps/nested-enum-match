pub enum Vertical {
    Up(usize),
    Zero,
    Down(usize),
}

pub enum Horizontal {
    Left(usize),
    Zero,
    Right(usize),
}

pub fn distance_nested(v: Vertical, h: Horizontal) -> usize {
    match v {
        Vertical::Up(dv) => match h {
            Horizontal::Left(dh) => dv + dh,
            Horizontal::Zero => dv,
            Horizontal::Right(dh) => dv + dh,
        },
        Vertical::Zero => match h {
            Horizontal::Left(dh) => dh,
            Horizontal::Zero => 0,
            Horizontal::Right(dh) => dh,
        },
        Vertical::Down(dv) => match h {
            Horizontal::Left(dh) => dv + dh,
            Horizontal::Zero => dv,
            Horizontal::Right(dh) => dv + dh,
        },
    }
}

pub fn distance_tuple(v: Vertical, h: Horizontal) -> usize {
    match (v, h) {
        (Vertical::Up(dv), Horizontal::Left(dh)) => dv + dh,
        (Vertical::Up(dv), Horizontal::Zero) => dv,
        (Vertical::Up(dv), Horizontal::Right(dh)) => dv + dh,
        (Vertical::Zero, Horizontal::Left(dh)) => dh,
        (Vertical::Zero, Horizontal::Zero) => 0,
        (Vertical::Zero, Horizontal::Right(dh)) => dh,
        (Vertical::Down(dv), Horizontal::Left(dh)) => dv + dh,
        (Vertical::Down(dv), Horizontal::Zero) => dv,
        (Vertical::Down(dv), Horizontal::Right(dh)) => dv + dh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn distance_nested(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                super::distance_nested(Vertical::Up(3), Horizontal::Left(4));
            }
        });
    }

    #[bench]
    fn distance_tuple(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                super::distance_tuple(Vertical::Up(3), Horizontal::Left(4));
            }
        });
    }
}
