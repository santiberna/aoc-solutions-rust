pub mod assembunny;
pub mod input;
pub mod tree;
pub mod modular;
pub mod hash;
pub mod matrix;

#[macro_export]
macro_rules! check_result {
    ($in:expr, $a1:expr, $a2:expr) => {
        #[test]
        fn check_results() {
            assert_eq!(challenge($in), ($a1, $a2))
        }
    };
}

#[macro_export]
macro_rules! check_result2 {
    ($a1:expr, $a2:expr) => {
        #[test]
        fn check_results() {
            assert_eq!(challenge(), ($a1, $a2))
        }
    };
}