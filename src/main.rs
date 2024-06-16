mod usb;
mod my_io;
mod user_operations;

use std::io::Result;
use crate::usb::Usb;
use crate::user_operations::UserOperations;

fn main() -> Result<()> {    
    let mut program = UserOperations::new();
    program.run();
    Ok(())
}

