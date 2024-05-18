use anyhow::Context;
use ecosystem::my_error::ValidateError;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    read_file("test.txt".to_string()).context("test.txt")?;
    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), ValidateError> {
    Err(ValidateError::InvalidArgument("argument".to_string()))
}

fn read_file(p: String) -> Result<(), ValidateError> {
    // can convert is because ValidateError is already have io error: IOError;
    fs::File::open(p)?;
    Ok(())
}
