pub enum Vertical {
    Up,
    Down,
}

pub enum Horizontal {
    Left,
    Right,
}

pub fn match_nested(v: Vertical, h: Horizontal) -> usize {
    match v {
        Vertical::Up => match h {
            Horizontal::Left => 0,
            Horizontal::Right => 1,
        },
        Vertical::Down => match h {
            Horizontal::Left => 2,
            Horizontal::Right => 3,
        },
    }
}

pub fn match_tuple(v: Vertical, h: Horizontal) -> usize {
    match (v, h) {
        (Vertical::Up, Horizontal::Left) => 0,
        (Vertical::Up, Horizontal::Right) => 1,
        (Vertical::Down, Horizontal::Left) => 2,
        (Vertical::Down, Horizontal::Right) => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn match_nested(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                super::match_nested(Vertical::Up, Horizontal::Left);
            }
        });
    }

    #[bench]
    fn match_tuple(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                super::match_tuple(Vertical::Up, Horizontal::Left);
            }
        });
    }
}
