
pub mod rustpi_io{
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::Result;
    use std::io::ErrorKind;
    use std::io::Error;
    use std::path::Path;
    use std::fmt;

    static GPIO_PATH: &'static str = "/sys/class/gpio/";

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum GPIOMode{
        Read,
        Write
    }

    #[derive(Debug, Clone, Copy)]
    pub enum GPIOData{
        Low = 0,
        High = 1
    }

    pub struct GPIO {
        pin: u8,
        mode: GPIOMode
    }

    impl GPIO{
        pub fn gpio_number(&self) -> u8 { self.pin }

        pub fn current_mode(&self) -> GPIOMode {
            self.mode
        }

        pub fn set_mode(&mut self, mode: GPIOMode) -> Result<&mut Self>{
            let mut direction = OpenOptions::new().write(true).open(format!("{}gpio{}/direction", GPIO_PATH, self.pin))?;
            match mode{
                GPIOMode::Read  => try!(direction.write_all("in".as_bytes())),
                GPIOMode::Write => try!(direction.write_all("out".as_bytes())),
            };
            self.mode = mode;
            Ok(self)
        }

        pub fn init(gpio: u8, mode: GPIOMode) -> Result<Self> {
            println!("gpio number = {}", gpio);
            println!("mode = {}", mode);
            if Path::new(&format!("{}gpio{}/", GPIO_PATH, gpio)).exists(){
                return Err(Error::new(ErrorKind::AddrInUse, "Error: gpio was already initialized"))
            }
            {
                let mut export = OpenOptions::new().write(true).open(format!("{}export", GPIO_PATH))?;
                export.write_all(format!("{}", gpio).as_bytes())?;
            }
            let mut result = GPIO{pin: gpio, mode: mode};
            result.set_mode(mode)?;
            Ok(result)
        }

        pub fn value(&self) -> Result<GPIOData>{
            let mut value = try!(OpenOptions::new().read(true).open(format!("{}gpio{}/value", GPIO_PATH, self.pin)));
            let mut buffer = vec![];
            try!(value.read_to_end(&mut buffer ));
            match buffer[0] as char{
                '0' => Ok(GPIOData::Low),
                '1' => Ok(GPIOData::High),
                _ => Err(Error::new(ErrorKind::InvalidData, "read value other than 1 or 0"))
            }
        }

        pub fn set(&self, data: GPIOData) -> Result<()>{
            if self.mode != GPIOMode::Write {
                return Err(Error::new(ErrorKind::PermissionDenied, "Error: gpio is not in write mode"))
            }
            let buffer = match data{
                GPIOData::Low => "0",
                GPIOData::High => "1"
            };
            let mut direction = OpenOptions::new().write(true).open(format!("{}gpio{}/value", GPIO_PATH, self.pin))?;
            try!(direction.write_all(buffer.as_bytes()));
            Ok(())
        }
    }

    impl Drop for GPIO{
        fn drop(&mut self){
            let mut unexport =
            match OpenOptions::new().write(true).open(format!("{}unexport", GPIO_PATH)) {
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

    impl fmt::Display for GPIOData {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                GPIOData::Low => write!(f, "LOW"),
                GPIOData::High => write!(f, "HIGH")
            }
        }
    }

    impl fmt::Display for GPIOMode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                GPIOMode::Read => write!(f, "Read"),
                GPIOMode::Write => write!(f, "Write")
            }
        }
    }
}
