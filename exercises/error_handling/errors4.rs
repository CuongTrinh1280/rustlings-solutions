// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Err(CreationError::Zero)
        }
    }

    fn check_non_zero(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            input if input < 0 => Err(CreationError::Negative),
            input if input == 0 => Err(CreationError::Zero),
            input => Ok(PositiveNonzeroInteger(input as u64)),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));

    assert!(PositiveNonzeroInteger::check_non_zero(100).is_ok());
}
