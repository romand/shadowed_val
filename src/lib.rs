// Clippy lints
// <https://rust-lang.github.io/rust-clippy/current/>
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::unnecessary_wraps)]

use std::fmt::Error;

const fn bar() -> Result<(), Error> {
    Ok(())
}

pub fn foo() -> Result<(), Error> {
    let val = 1;
    bar()?;
    println!("{}", val);
    Ok(())
}
