
pub mod rustpi_io{
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::error::Error;
    use std::io::Result;


    static GPIO_PATH: &'static str = "/sys/class/gpio/";

    #[derive(Debug, Clone, Copy)]
    pub enum GPIOMode{
        Read,
        Write
    }

    pub struct GPIO {
        pin: u8,
        mode: GPIOMode
    }

    impl GPIO{
        pub fn gpio(self) -> u8 { self.pin }

        pub fn current_mode(self) -> GPIOMode {
            self.mode
        }

        pub fn set_mode(self, mode: GPIOMode) -> Result<Self>{
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
            };
            Ok(Self { pin: self.pin, mode: mode})
        }

        pub fn init(gpio: u8, mode: GPIOMode) -> Result<Self> {
            println!("gpio number = {:?}", gpio);
            println!("mode = {:?}", mode);
            {
                let mut export_file = match OpenOptions::new().write(true).open(format!("{}export", GPIO_PATH)) {
                    Ok(f) => f,
                    Err(e) => panic!("file error: {}", e),
                };
                match export_file.write_all(format!("{}", gpio).as_bytes()) {
                    Err(why) => {
                       panic!("couldn't initialize gpio {}: {}",gpio , why)
                    },
                    Ok(_) => println!("gpio {} initialized", gpio),
                }
           }
           Ok(GPIO{pin: gpio, mode: mode}.set_mode(mode)?)
        }
    }

    impl Drop for GPIO{
        fn drop(&mut self){
            let mut unexport = match OpenOptions::new().write(true).open(format!("{}unexport", GPIO_PATH)) {
                Ok(f) => f,
                Err(e) => panic!("file error: {}", e),
            };
            match unexport.write_all(format!("{}", self.pin).as_bytes()) {
                Err(why) => {
                   panic!("couldn't close gpio {}: {}",self.pin , why)
                },
                Ok(_) => println!("gpio {} closed", self.pin),
            }
        }
    }

    impl Read for GPIO {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize>{
            let mut value = match OpenOptions::new().read(true).open(format!("{}gpio{}/value", GPIO_PATH, self.pin)) {
                Ok(f) => f,
                Err(e) => panic!("file error: {}", e),
            };
            let mut buffer = vec![];
            match value.read_to_end(&mut buffer ){
                Ok(n) => {
                    buf[0] = buffer[0];
                    Ok(n)
                },
                Err(e) => panic!(e),
            }
        }
    }
}
