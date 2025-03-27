fn ex_1() {
    let mut cereals: [&str; 5] = [
        "Cookie Crisp",
        "Cinnamon Toast Crunch",
        "Frosted Flakes",
        "Cocoa Puffs",
        "Captain Crunch",
    ];
    let first_two: &[&str] = &cereals[..2];
    println!("{first_two:?}");
    let mid_three: &[&str] = &cereals[1..4];
    println!("{mid_three:?}");
    let last_thee: &mut [&str] = &mut cereals[2..];
    last_thee[2] = "Lucky Charms";
    println!("{cereals:?}");
    let cookie_crisp: String = String::from(cereals[0]);
    println!("{cookie_crisp}");
    let cookie: &str = &cookie_crisp[..6];
    println!("{cookie}");
    let cook_puffs: &String = &String::from(cereals[3]);
    println!("{cook_puffs}");
    let pufs: &str = &cook_puffs[6..];
    println!("{pufs}");
}
