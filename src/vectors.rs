#[derive(Debug)]
struct File {
    name: String,
}
#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}
impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            contents: Vec::new(),
        }
    }
    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }
    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }
    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}
fn main() {
    let mut folder = Folder::new("hello".to_string());
    folder.create_file("1".to_string());
    folder.create_file("2".to_string());
    folder.create_file("3".to_string());
    println!("{:#?}", folder);
    let file_1 = folder.delete_file(0);
    println!("Deleted file - {:#?}", file_1);
    println!("{:#?}, folder name: {}", folder, folder.name);
    if let Some(f) = folder.get_file(0) {
        println!("Get file - {:#?}", f.name);
    } else {
        println!("No file found");
    }

    //or
    match folder.get_file(0) {
        Some(f) => {
            println!("Get file - {:#?}", f.name);
        }
        None => println!("No file found"),
    }
}
