
mod rustpi_io{
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::error::Error;

    static GPIO_PATH: &'static str = "/sys/class/gpio/";

    #[derive(Debug)]
    pub enum GPIOMode{
        Read,
        Write
    }

    pub struct GPIO {
        pin: u8,
        mode: GPIOMode
    }

    impl GPIO{
        pub fn gpio(&self) -> u8 { self.pin }

        pub fn currentMode(&self) -> GPIOMode {
            match self.mode {
                GPIOMode::Read => GPIOMode::Read,
                GPIOMode::Write => GPIOMode::Write
            }
        }

        pub fn setMode(&self, mode: GPIOMode) -> GPIO {
            let mut direction_file = match OpenOptions::new().write(true).open(format!("{}gpio{}/direction", GPIO_PATH, self.pin)) {
                Ok(f) => f,
                Err(e) => panic!("file error: {}", e),
            };

            match mode{
                GPIOMode::Read   => {
                    match direction_file.write_all("in".as_bytes()) {
                        Err(why) => {
                            panic!("couldn't open direction file for gpio {}: {}",self.pin , why.description())
                        },
                        Ok(_) => println!("gpio {} set to read mode", self.pin),
                    }
                }
                GPIOMode::Write   => {
                    match direction_file.write_all("out".as_bytes()) {
                        Err(why) => {
                            panic!("couldn't open direction file for gpio {}: {}",self.pin , why.description())
                        },
                        Ok(_) => println!("gpio {} set to write mode", self.pin),
                    }
                }
            }
            GPIO {pin: self.pin, mode: mode}
        }

        pub fn init(gpio: u8, mode: GPIOMode) -> GPIO {
            println!("path = {:?}", GPIO_PATH);
            println!("gpio number = {:?}", gpio);
            println!("mode = {:?}", mode);

            {
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
           {

           }
           GPIO {pin: gpio, mode: mode}
        }
    }
}
