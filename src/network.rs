pub fn fetch(url: String) -> String {
    println!("Loading URL...\r");
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("URL loaded!\r");
    return response;
}
