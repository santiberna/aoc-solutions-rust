#[macro_export]
macro_rules! check_result {
    ($in:expr, $a1:expr, $a2:expr) => {
        #[test]
        fn check_results() {
            assert_eq!(challenge($in), ($a1, $a2))
        }
    };
}

pub mod math {

    use num::Num;
    use std::ops::{Add, Div, Mul, Neg, Sub};

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Vec2<T>
    where
        T: Copy + Num + Neg<Output = T>,
    {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }

        pub fn turn_right(&self) -> Self {
            Self {
                x: self.y,
                y: -self.x,
            }
        }

        pub fn turn_left(&self) -> Self {
            Self {
                x: -self.y,
                y: self.x,
            }
        }
    }

    // Vector + Vector
    impl<T> Add for Vec2<T>
    where
        T: Num + Copy + Neg<Output = T>,
    {
        type Output = Vec2<T>;
        fn add(self, rhs: Vec2<T>) -> Vec2<T> {
            Vec2::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    // Vector - Vector
    impl<T> Sub for Vec2<T>
    where
        T: Num + Copy + Neg<Output = T>,
    {
        type Output = Vec2<T>;
        fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
            Vec2::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    // Vector * Scalar
    impl<T> Mul<T> for Vec2<T>
    where
        T: Num + Copy + Neg<Output = T>,
    {
        type Output = Vec2<T>;
        fn mul(self, rhs: T) -> Vec2<T> {
            Vec2::new(self.x * rhs, self.y * rhs)
        }
    }

    // Vector / Scalar
    impl<T> Div<T> for Vec2<T>
    where
        T: Num + Copy + Neg<Output = T>,
    {
        type Output = Vec2<T>;
        fn div(self, rhs: T) -> Vec2<T> {
            Vec2::new(self.x / rhs, self.y / rhs)
        }
    }

    // -Vector (negation)
    impl<T> Neg for Vec2<T>
    where
        T: Num + Copy + Neg<Output = T>,
    {
        type Output = Vec2<T>;
        fn neg(self) -> Vec2<T> {
            Vec2::new(-self.x, -self.y)
        }
    }
}
