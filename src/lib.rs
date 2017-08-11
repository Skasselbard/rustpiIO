static GPIO_PATH: &'static str = "/sys/class/gpio/";

pub fn init() {
    println!("{:?}", GPIO_PATH);
}
