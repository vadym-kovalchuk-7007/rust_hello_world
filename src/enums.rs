enum Laundry {
    Cold,
    Hot(String),
    Delicate { temp: u8 },
}

impl Laundry {
    fn notify(&self) {
        match self {
            Laundry::Cold => {
                println!("Is cold");
            }
            Laundry::Hot(msg) => {
                println!("Laundry is too hot {msg}");
            }
            Laundry::Delicate { temp } => {
                println!("Delicate with {temp}");
            }
        }
    }
}

fn main() {
    Laundry::Cold.notify();
    let laundry_hot = Laundry::Hot(String::from("Too dirty"));
    laundry_hot.notify();
    let laundry_delicate = Laundry::Delicate { temp: 10 };
    laundry_delicate.notify();
}
