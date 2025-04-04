#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(it) => it,
            MyOption::None => panic!("Oh"),
        }
    }
    fn unwrap_or(self, default_value: i32) -> i32 {
        match self {
            MyOption::Some(it) => it,
            MyOption::None => default_value,
        }
    }
}

fn divider(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("denominator cannot be zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn check_divider(result: Result<f64, String>) {
    match result {
        Ok(it) => println!("The divider is {}", it),
        Err(it) => println!("The err message is: {}", it),
    }
}

fn let_while_option() {
    let mut souses: Vec<&str> = vec!["Ketchup", "Mushroom", "Cheese"];
    while let Some(souse) = souses.pop() {
        println!("{}", souse);
    }
}

#[derive(Debug)]
struct Food {
    name: String,
}
#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}
impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }
        if self.reservations < 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err("Sorry, we have a mice problem".to_string());
        }
        if address.is_empty() {
            return Err("Delivery address is empty".to_string());
        }
        Ok(Food {
            name: String::from("Burger"),
        })
    }
}

fn main() {
    let my_option = MyOption::Some(1);
    let none_option = MyOption::None;
    println!("{:?}", my_option.unwrap());
    // println!("{:?}", none_option.unwrap());
    println!("unwrap_or {:?}", my_option.unwrap_or(0));
    println!("unwrap_or default value {:?}", none_option.unwrap_or(0));

    let result = divider(10.0, 1.2);
    println!("{:?}", result);
    check_divider(result);
    let result = divider(10.0, 0.0);
    println!("{:?}", result);
    check_divider(result);

    let_while_option();
    //Restaurant
    let rest = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    if let Some(it) = rest.chef_special() {
        println!("Special {it:?}")
    } else {
        println!("None found")
    };
    //OR
    println!("{:?}", rest.chef_special());
    println!("Deliver burger {:?}", rest.deliver_burger("123 Elm Street"));
    let rest_2 = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("Chef Spec {:?}", rest_2.chef_special());
    println!(
        "Burger deliv rest 2 {:?}",
        rest_2.deliver_burger("123 Elm Street")
    );
    println!("Burger deliv rest 2 {:?}", rest_2.deliver_burger(""));
}
