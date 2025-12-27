trait Device {
    fn open(&mut self);
    fn read(&self) -> String;
    fn write(&mut self, data: &str);
    fn close(&mut self);
}

struct CharDevice {
    buffer: String,
    is_open: bool,
}

impl Device for CharDevice {
    fn open(&mut self) {
        self.is_open = true;
        println!("Device opened");
    }

    fn write(&mut self, data: &str) {
        if self.is_open {
            self.buffer.push_str(data);
        }
    }

    fn read(&self) -> String {
        if self.is_open {
            self.buffer.clone()
        } else {
            String::from("Device not open")
        }
    }

    fn close(&mut self) {
        self.is_open = false;
        println!("Device closed");
    }
}

fn main() {
    let mut dev = CharDevice {
        buffer: String::new(),
        is_open: false,
    };

    dev.open();
    dev.write("Hello Device");
    println!("{}", dev.read());
    dev.close();
}

