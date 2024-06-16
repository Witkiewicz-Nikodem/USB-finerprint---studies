use std::io;
use serde_json;
use std::fs::File;
use std::io::prelude::*;

use crate::usb::Usb;


pub fn write_usbs(usbs: &Vec<Usb>, file_name: &String) -> io::Result<()>{
    let serialised_usbs = serde_json::to_string(&usbs)?;
    let mut file = File::create(file_name)?;
    file.write_all(serialised_usbs.as_bytes())?;
    Ok(())
}

pub fn read_usbs(file_name: &String) -> io::Result<Vec<Usb>>{
    let mut file = match File::open(file_name){
        Ok(file) => file,
        Err(e) => return Err(e.into()),
    };
    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;
    let usbs: Vec<Usb> = serde_json::from_str(&file_data)?;
    Ok(usbs)
}