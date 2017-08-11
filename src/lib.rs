use std::fs::OpenOptions;
//use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


static GPIO_PATH: &'static str = "/sys/class/gpio/";

#[derive(Debug)]
pub enum GPIOMode{
    Read,
    Write
}

pub fn init(gpio: u8, mode: GPIOMode) {
    println!("path = {:?}", GPIO_PATH);
    println!("gpio number = {:?}", gpio);
    println!("mode = {:?}", mode);

    let mut export_file = match OpenOptions::new().write(true).open(format!("{}export", GPIO_PATH)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

   match export_file.write_all(format!("{}", gpio).as_bytes()) {
       Err(why) => {
           panic!("couldn't initialize gpio {}: {}",gpio , why.description())
       },
       Ok(_) => println!("gpio {} initialized", gpio),
   }
}
