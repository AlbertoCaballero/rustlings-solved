// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(i64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 { 
           return Ok(PositiveNonzeroInteger(value as i64));
        } else if value < 0{
            return Err(CreationError::Negative);
        } else {
            return Err(CreationError::Zero);
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
}
