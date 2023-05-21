use reqwest;
use serde::Deserialize;


pub struct Pokeapi {
    httpClient: reqwest::Client,
}

#[derive(Deserialize)]
pub struct ResDestructure {
    count: i32,
    previous: String,
    next: String,
}


impl ResDestructure {
    pub fn new(count: i32, previous: &str, next: &str) -> Self {
        ResDestructure {
            count,
            previous: previous.to_string(),
            next: next.to_string(),
        }
    }

    pub fn json_str() {
        let json = r#"{
            "count": 20,
            "current": "https://pokeapi.co/api/v2/location-area/?limit=60&offset=20",
            "next": "https://pokeapi.co/api/v2/location-area/?limit=60&offset=80"
        }"#;
        let my_struct: ResDestructure = serde_json::from_str(json).unwrap();
    }
    
}

#[tokio::main]
pub async fn get() -> Result<(), reqwest::Error> {
    let url = "https://pokeapi.co/api/v2/location-area/?limit=60&offset=20";
    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    println!("Response body:\n{}", body);

    Ok(())
}