use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

async fn _read_and_parse(file_name: &str) -> String {
    let mut file = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await.unwrap();
    let config = String::from_utf8(buf).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&config).unwrap();
    let api_key = parsed.get("api_key").unwrap();
    api_key.to_string()
}

async fn _read_and_parse_better(file_name: &str) -> Result<String, &'static str> {
    let mut file = File::open(file_name).await.map_err(|_| "Failed to open file")?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await.map_err(|_| "Failed to read file")?;
    let config = String::from_utf8(buf).map_err(|_| "Failed to parse file")?;
    let parsed: serde_json::Value = serde_json::from_str(&config).map_err(|_| "Failed to parse file")?;
    match parsed.get("api_key") {
        Some(api_key) => Ok(api_key.to_string()),
        None => Err("Failed to find api_key in configuration file")
    }
}

async fn read_and_parse_better_anyhow(file_name: &str) -> anyhow::Result<String> {
    let mut file = File::open(file_name).await?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await?;
    let config = String::from_utf8(buf)?;
    let parsed: serde_json::Value = serde_json::from_str(&config)?;
    match parsed.get("api_key") {
        Some(api_key) => Ok(api_key.to_string()),
        None => anyhow::bail!("Failed to find api_key in configuration file")
    }
    //Ok(parsed.get("api_key").ok_or(anyhow::anyhow!("Failed to find api_key in configuration file"))?.to_string())
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Unknown error")]
    Unkown
}

async fn read_and_parse_better_best(file_name: &str) -> Result<String, ParseError> {
    let mut file = File::open(file_name).await?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await?;
    let config = String::from_utf8(buf)?;
    let parsed: serde_json::Value = serde_json::from_str(&config)?;
    match parsed.get("api_key") {
        Some(api_key) => Ok(api_key.to_string()),
        None => Err(ParseError::Unkown)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let config = read_and_parse("configuration.json").await;
    // println!("{}", config);

    // let config = read_and_parse_better("configurtion.json").await;
    // match config {
    //     Ok(config) => println!("{}", config),
    //     Err(e) => println!("ERROR: {}", e)
    // }
    
    // let config = read_and_parse_better_anyhow("configuration.json").await;
    // match config {
    //     Ok(config) => println!("{}", config),
    //     Err(e) => {
    //         if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
    //             println!("IO Error: {}", io_err);
    //         } else if let Some(serde_err) = e.downcast_ref::<serde_json::Error>() {
    //             println!("Serde Error: {}", serde_err);
    //         } else {
    //             println!("Unknown Error: {}", e);
    //         }
    //     }
    // }

    let config = read_and_parse_better_best("configuration.json").await;
    match config {
        Ok(config) => println!("{}", config),
        Err(e) => {
            match e {
                ParseError::Io(e) => println!("IO Error: {}", e),
                ParseError::Utf8(e) => println!("UTF-8 Error: {}", e),
                ParseError::Json(e) => println!("JSON Error: {}", e),
                ParseError::Unkown => println!("Unknown Error")
            }
        }
    }

    Ok(())
}


