use serde::Deserialize;

#[derive(Deserialize)]
struct ReleaseResponse {
    tag_name: String,
}

const RELEASES_API_URL: &'static str = "https://api.github.com/repos/Anduin2017/HowToCook/releases";

pub fn get_releases(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(RELEASES_API_URL)
        .header("User-Agent", username)
        .send()?
        .json::<Vec<ReleaseResponse>>()?;
    let res = response.first().ok_or_else(|| "No releases found")?;
    Ok(res.tag_name.clone())
}

mod tests {
    #[test]
    fn test_get_releases() {
        let username = whoami::username().unwrap_or("Unknown".to_string());
        match super::get_releases(&username) {
            Ok(tag_name) => println!("Latest release tag: {}", tag_name),
            Err(e) => eprintln!("Error fetching releases: {}", e),
        }
    }
}
