use std::ops::Add;

struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    }
}

let fast = Speed(5) + Speed(3);