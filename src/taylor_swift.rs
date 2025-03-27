#[derive(Debug)]
struct TaylorSwift {
    title: String,
    duration: u32,
    year: u32,
}

impl TaylorSwift {
    fn new(title: String, duration: u32, year: u32) -> Self {
        Self {
            title,
            duration,
            year,
        }
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

fn ex_struct() {
    let mut song_1 = TaylorSwift::new(String::from("Some title"), 231, 2014);
    println!("{}", &song_1.get_title());
    song_1.set_title(String::from("New title"));
    println!(
        "{} {} {}",
        &song_1.get_title(),
        &song_1.year,
        &song_1.duration
    );
}
