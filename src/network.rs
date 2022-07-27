pub fn fetch(url: String) -> String {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    return response;
}
