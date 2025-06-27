// https://doc.rust-lang.org/rust-by-example/meta/doc.html
enum Color {
    Gold,
    Gray,
    Black,
    Calico,
    Unknown,
}

impl Color {
    fn get_color(&self) -> String {
        let color: &str;
        match self {
            Color::Gold => {
                color = "Gold";
            }
            Color::Gray => {
                color = "Gray";
            }
            Color::Black => {
                color = "Black";
            }
            Color::Calico => {
                color = "Calico";
            }
            Color::Unknown => {
                color = "Unknown";
            }
        }
        color.to_string()
    }
}

// #[derive(Debug)]
enum Sex {
    Girl,
    Boy,
    Unknown,
}

impl Sex {
    fn get_sex(&self) -> String {
        let sex: &str;
        match self {
            Sex::Girl => {
                sex = "Girl";
            }
            Sex::Boy => {
                sex = "Boy";
            }
            Sex::Unknown => {
                sex = "They";
            }
        }
        sex.to_string()
    }
}

struct Cat<T, U> {
    name: String,
    age: f32,
    color: T,
    is_healthy: bool,
    sex: U,
}

enum Messages {
    Error(String),
    Notify { msg: String, title: String },
}

impl Messages {
    fn get_msg(e: Messages) -> String {
        match e {
            Messages::Error(e) => e.to_string(),
            Messages::Notify { msg, title } => {
                let t_delim: [char; 2] = ['[', ']'];
                // let mut s = String::from(title);
                // s.insert(0, t_delim[0]);
                // s.insert(s.len(), t_delim[1]);
                // s.push_str(&msg);
                // s
                String::from(format!("{}{}{}{}", t_delim[0], title, t_delim[1], msg))
            }
        }
    }

    fn append_msg<'a>(&mut self, text: &'a str) -> &mut Self {
        match self {
            Messages::Notify { msg, title: _ } => {
                msg.push_str(text);
            }
            Messages::Error(e) => {
                e.push_str(text);
            }
        }
        self
    }
}

impl Cat<Color, Sex> {
    fn new(color: Color, sex: Sex) -> Cat<Color, Sex> {
        Cat {
            name: String::new(),
            age: 0.1,
            color,
            is_healthy: true,
            sex,
        }
    }

    fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    fn set_health(&mut self, is_healthy: bool) -> &mut Self {
        self.is_healthy = is_healthy;
        self
    }

    fn check_health(&self) -> &str {
        if let true = self.is_healthy {
            "Good"
        } else {
            "Bad"
        }
    }

    fn set_sex(&mut self, sex: Sex) -> &mut Self {
        self.sex = sex;
        self
    }

    fn set_age(&mut self, age: f32) -> Result<bool, Messages> {
        if self.age < age {
            self.age = age;
            Ok(true)
        } else {
            let mut err = Messages::Error("Error: ".to_string());
            err.append_msg(&format!("The new age {} is less than existing", &self.name));
            Err(err)
        }
    }

    fn voice(&self) {
        println!("{} say Myaoo", self.name);
    }

    fn show(&self) -> &Self {
        let mut notif = Messages::Notify {
            title: String::from("Cat info:"),
            msg: String::new(),
        };
        notif
            .append_msg(&format!("\n Name:{}", &self.name))
            .append_msg(&format!(", Color:{}", &self.color.get_color()))
            .append_msg(&format!(", Sex:{}", &self.sex.get_sex()))
            .append_msg(&format!(", Age:{}", &self.age.to_string()))
            .append_msg(&format!(", Health:{}", &self.check_health()));
        println!("{}", Messages::get_msg(notif));
        self
    }
}

struct CatFamily {
    name: String,
    parents: Vec<Cat<Color, Sex>>,
    children: Vec<Cat<Color, Sex>>,
}

impl CatFamily {
    fn new(name: String, parents: Vec<Cat<Color, Sex>>) -> Self {
        CatFamily {
            name,
            parents,
            children: Vec::new(),
        }
    }

    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) -> Result<String, String> {
        let min_len = 3usize;
        if name.len() > min_len {
            return Ok(String::from(format!("{}->{}", self.name, name)));
        }
        Err(String::from(format!(
            "Length should be greater than {}",
            min_len
        )))
    }

    fn add_child(&mut self, child: Cat<Color, Sex>) -> &mut Self {
        self.children.push(child);
        self
    }

    fn exit_child(&mut self, index: usize) -> Cat<Color, Sex> {
        self.children.remove(index)
    }

    fn get_child(&mut self, index: usize) -> Option<&mut Cat<Color, Sex>> {
        if index < self.children.len() {
            return Some(&mut self.children[index]);
        }
        None
        //self.children.get(index) //Return reference Option<&Cat<_,_>>
    }

    fn create_child(catty: (Color, Sex, &'static str)) -> Cat<Color, Sex> {
        let mut cat_child = Cat::new(catty.0, catty.1);
        cat_child.set_name(String::from(catty.2)); // .show();
        cat_child
    }

    fn get_mom_dad(parent: &Cat<Color, Sex>) -> &str {
        if let Sex::Girl = parent.sex {
            "Mom"
        } else {
            "Dad"
        }
    }

    fn print_parents(&self) -> &Self {
        let mut title = String::from("Parents. Cat Family: ");
        title.push_str(&self.get_name());
        let mut notif = Messages::Notify {
            title,
            msg: String::new(),
        };
        for parent in &self.parents {
            notif
                .append_msg(&format!(
                    "\n {}:{}",
                    CatFamily::get_mom_dad(&parent),
                    &parent.name
                ))
                .append_msg(&format!(", Age:{}", &parent.age))
                .append_msg(&format!(", Color:{}", &parent.color.get_color()))
                .append_msg(&format!(", Health:{}", parent.check_health()));
        }
        println!("{}", Messages::get_msg(notif));
        self
    }

    fn print_children(&mut self) {
        let mut title = String::from("Children. Cat Family: ");
        title.push_str(&self.get_name());
        let msg = if 0 < self.children.len() {
            String::new()
        } else {
            String::from("\n 0")
        };
        let mut children: Messages = Messages::Notify { title, msg };
        for index in 0..self.children.len() {
            if let Some(c) = &self.get_child(index) {
                children
                    .append_msg(&format!("\n {}", &(index + 1).to_string()))
                    .append_msg(&format!(" Name:{}", &c.name))
                    .append_msg(&format!(", Color:{}", &c.color.get_color()))
                    .append_msg(&format!(", Age:{}", &c.age.to_string()))
                    .append_msg(&format!(", Health:{}", &c.check_health()));
                if index < self.children.len() - 1 {
                    children.append_msg(", ");
                }
            }
        }
        println!("{}", Messages::get_msg(children));
    }

    fn show(&mut self) -> &mut Self {
        self.print_parents();
        self.print_children();
        self
    }
}

fn get_catties() -> [(Color, Sex, &'static str); 5] {
    [
        (Color::Gray, Sex::Girl, "Mommy Gray Cat"),
        (Color::Gray, Sex::Boy, "Wild Gray Cat"),
        (Color::Gray, Sex::Boy, "Light Gray"),
        (Color::Calico, Sex::Girl, "Gurza"),
        (Color::Black, Sex::Boy, "Cygan"),
    ]
}

fn main() {
    let mut cat_mom: Cat<Color, Sex> = Cat::new(Color::Unknown, Sex::Unknown);
    cat_mom
        .set_health(true)
        .set_sex(Sex::Girl)
        .set_name(String::from("Gray Pregant cat"))
        .set_color(Color::Gray);
    if let Err(it) = cat_mom.set_age(1.0) {
        println!("{}", Messages::get_msg(it)); //Error: The new age Gray Pregant cat is less than existing
    }
    cat_mom.show().voice(); //Cat { name: "Gray Pregant cat", age: 2.0, color: Gray, is_healthy: true, sex: Girl }
    //Gray Pregant cat say Myaoo

    let mut cat_dad: Cat<Color, Sex> = Cat::new(Color::Gold, Sex::Boy);
    if let Err(it) = cat_dad
        .set_health(true)
        .set_name(String::from("Major Golden Cat"))
        .set_age(2.0)
    {
        // .expect("Can't set dad age");
        println!("{}", Messages::get_msg(it)); //Error:The new age is less than existing
    }
    cat_dad.show().voice(); //Cat { name: "Major Golden Cat", age: 2.0, color: Gold, is_healthy: true, sex: Boy }
    //Major Golden Cat say Myaoo

    let mut cat_family = CatFamily::new(String::from("Yurk Jbruly"), vec![cat_mom, cat_dad]);
    cat_family.print_parents();
    //[Parents. Cat Family: Yurk Jbruly]
    //  Mom:Gray Pregant cat, Age:2, Color:Gray, Health:Good
    //  Dad:Major Golden Cat, Age:2, Color:Gold, Health:Good

    //let catt = cat_family.get_child(0); //catt = None
    cat_family.show();
    // [Parents. Cat Family: Yurk Jbruly]
    //  Mom:Gray Pregant cat, Age:2, Color:Gray, Health:Good
    //  Dad:Major Golden Cat, Age:2, Color:Gold, Health:Good
    // [Children. Cat Family: Yurk Jbruly]
    //  0

    for catty in get_catties() {
        let cat_child = CatFamily::create_child(catty);
        cat_family.add_child(cat_child);
    }
    let cygan_indx = 4usize;
    if let Some(_c) = cat_family.get_child(cygan_indx) {
        //c
        //.show() //[Cat info:] name: "Cygan", age: 0.1, color: Black, is_healthy: true, sex: Boy
        //.voice(); //Cygan say Myaoo
        let _cygan: Cat<Color, Sex> = cat_family.exit_child(cygan_indx);
        // _cygan.show(); // Cat { name: "Cygan", age: 0.1, color: Black, is_healthy: true, sex: Boy }
    } else {
        println!("index {cygan_indx} not exists in CatFamily");
    }
    //cat_family.show(); //CatFamily { name: "Yurk Jbruly", parents: [Cat { name: "Gray Pregant cat", age: 2.0, color: Gray, is_healthy: true, sex: Girl }, Cat { name: "Major Golden Cat", age: 2.0, color: Gold, is_healthy: true, sex: Boy }], children: [Cat { name: "Mommy Gray Cat", age: 0.1, color: Gray, is_healthy: true, sex: Girl }, Cat { name: "Wild Gray Cat", age: 0.1, color: Gray, is_healthy: true, sex: Boy }, Cat { name: "Light Gray", age: 0.1, color: Gray, is_healthy: true, sex: Boy }, Cat { name: "Gurza", age: 0.1, color: Black, is_healthy: true, sex: Boy }] }
    //cat_family.print_name();
    cat_family.print_children();
    // [Children. Cat Family: Yurk Jbruly]
    //  1 Name:Mommy Gray Cat, Color:Gray, Age:0.1, is healty:Good,
    //  2 Name:Wild Gray Cat, Color:Gray, Age:0.1, is healty:Good,
    //  3 Name:Light Gray, Color:Gray, Age:0.1, is healty:Good,
    //  4 Name:Gurza, Color:Calico, Age:0.1, is healty:Good
    let ren_result = match cat_family.set_name(String::from("Yurk Jbruliky")) {
        Ok(r) => r,
        Err(e) => {
            let err_msg = Messages::Error(format!("Error:{}", e).to_string());
            Messages::get_msg(err_msg) //Error:Length should be greater than 3
        }
    };
    println!("{ren_result}"); //Yurk Jbruly->Yurk Jbruliky
    cat_family.print_name();

    if let Some(catty) = cat_family.get_child(0) {
        if let Err(it) = catty.set_age(1.1) {
            println!("Error:{}", Messages::get_msg(it)); //Error:The new age is less than existing
        }
        catty
            .set_name(String::from("Mommies dark Gray Cat"))
            .set_health(false)
            .set_color(Color::Black)
            .set_sex(Sex::Boy)
            .show() //Cat { name: "Mommies dark Gray Cat", age: 1.1, color: Black, is_healthy: false, sex: Boy }
            .voice(); //Mommies dark Gray Cat say Myaoo
    }
    cat_family.print_children();
    // Cat Family Yurk Jbruliky children:
    // 1 Name:Mommies dark Gray Cat, Color:Black, Age:1.1, is healty:Bad,
    // 2 Name:Wild Gray Cat, Color:Gray, Age:0.1, is healty:Good,
    // 3 Name:Light Gray, Color:Gray, Age:0.1, is healty:Good,
    // 4 Name:Gurza, Color:Calico, Age:0.1, is healty:Good
}
