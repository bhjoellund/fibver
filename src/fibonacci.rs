use crate::models::AppErrors;

pub fn next_fibo_number(nr: i32, fib: i32, prev: i32) -> Result<i32, AppErrors> {
    if nr == fib {
        Ok(fib + prev)
    } else if nr < fib {
        Err(AppErrors::InvalidFibonacciNumber(nr))
    } else {
        next_fibo_number(nr, fib + prev, fib)
    }
}