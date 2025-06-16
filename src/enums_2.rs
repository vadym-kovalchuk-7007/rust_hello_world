#[derive(Debug)]

enum Laundry {
    Cold,
    Hot(u8),
    Delicate { program: String, temp: u8 },
}

impl Laundry {
    fn notify(&self) {
        let initial_msg = "Starting ";
        match self {
            Laundry::Cold => {
                println!("{initial_msg} cold");
            }
            Laundry::Hot(temp) => {
                println!("{initial_msg} hot on temp:{temp}");
            }
            Laundry::Delicate { program, temp } => {
                println!("{initial_msg} Delicate on program:{program} with temp:{temp}");
            }
        }
    }

    fn change_hot_temp(&mut self, n_temp: u8) -> Result<u8, String> {
        match *self {
            Laundry::Hot(temp) => {
                *self = Laundry::Hot(n_temp);
                Ok(temp)
            }
            _ => Err("Can't change".to_string()),
        }
    }

    fn change_delicate_temp(&mut self, n_temp: u8) -> Result<u8, String> {
        match self {
            Laundry::Delicate { program, temp } => {
                let temp = *temp;
                *self = Laundry::Delicate {
                    temp: n_temp,
                    program: program.to_string(),
                };
                Ok(temp)
            }
            _ => Err("Can't change delicate temp".to_string()),
        }
    }
}

fn main() {
    let l_cold = Laundry::Cold;
    let mut l_hot = Laundry::Hot(45);
    let mut l_delicate = Laundry::Delicate {
        program: String::from("hand-wash"),
        temp: 35,
    };
    l_cold.notify();
    l_hot.notify();
    l_delicate.notify();

    let res = l_hot.change_hot_temp(55);
    println!("{res:?}");
    l_hot.notify();

    let res_delicate_err = l_delicate.change_hot_temp(55);
    println!("{res_delicate_err:?}");
    l_delicate.notify();
    let res_delicate_err = l_hot.change_delicate_temp(40);
    println!("{res_delicate_err:?}");
    println!("{:?}", l_delicate.change_delicate_temp(40));
    l_delicate.notify();
}