use reqwest::blocking as rq;

fn main() {
    let client = rq::Client::new();
    let response = client.get("https://github.com/ninja-build/ninja/releases/download/v1.11.1/ninja-mac.zip").send().unwrap();
    let path = std::path::Path::new("bin/ninja-mac.zip");
    let mut file = std::fs::File::create(path).unwrap();
    let mut content = std::io::Cursor::new(response.bytes().unwrap());
    std::io::copy(&mut content, &mut file).unwrap();
}
