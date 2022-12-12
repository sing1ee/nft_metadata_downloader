use std::env;

use std::io::Cursor;

async fn fetch_url(url: String, file_name: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    pub name: String,
    pub image: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        
        let base_uri = &args[1];
        let category = &args[2];
        let dir = &args[3];
        let mut i = 1;
        while i <= 100 {
            let full_uri = format!("{}/{}", base_uri, i);
            println!("{}", full_uri);
            let resp: Metadata = reqwest::get(full_uri)
            .await?
            .json::<Metadata>()
            .await?;
            println!("{:#?}", resp);
            let file_name = format!("{}/{}/{}.png", dir, category, i);
            println!("write to {}", file_name);
            fetch_url(resp.image, file_name).await?;
            i += 1;
        }
    } else {
        println!("need one arg: baseUri");
    }
    Ok(())
    
}