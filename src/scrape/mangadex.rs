use std::error::Error;
use serde_json::Value;
// use fantoccini::{Client, ClientBuilder};
// use serde_json::{json, Value};
use tauri::{api::http::{ClientBuilder, HttpRequestBuilder, ResponseType}, http::header::USER_AGENT};
// use scraper::{selectable::Selectable, Html, Selector};

pub async fn get_covers(title: &str) -> Result<Value, Box<dyn Error>> {
  let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
  let url = format!("https://api.mangadex.org/manga?limit=100&includes[]=cover_art&title={}", title);
  let client = ClientBuilder::new()
    .max_redirections(3)
    .build()?;
  let request = HttpRequestBuilder::new("GET", url)?
    .header(USER_AGENT, user_agent)?
    .response_type(ResponseType::Text);
  
  // Sends the request and gets the data
  let res = client.send(request).await?;
  Ok(res.read().await?.data)
}

pub fn parse_covers(data: Value) -> Result<Vec<Value>, Box<dyn Error>> {
  let result = Vec::new();
  let d = data["data"].as_array().unwrap();

  for i in d {
    let id = i["id"].as_str().unwrap();
    let attributes = i.get("attributes").unwrap();
    let name = attributes["titles"]["en"].as_str().unwrap();
  }

  Ok(result)
}

/* 
async fn prep_client() -> Result<Client, Box<dyn Error>> {
  let mut caps = serde_json::Map::new();
  caps.insert("moz:firefoxOptions".to_string(), json!({ "args": ["-headless"] }));
  
  let c = ClientBuilder::native()
    .capabilities(caps)
    .connect("http://localhost:4444").await?;
  Ok(c)
}

// todo!("Need to go through all pages");
pub async fn get_covers(link: &str) -> Result<String, Box<dyn Error>> {
    // let c = CLIENT.get_or_init(async {
    //   let client = prep_client().await.unwrap();
    //   client
    // }).await;
    let mut caps = serde_json::Map::new();
    caps.insert("moz:firefoxOptions".to_string(), json!({ "args": ["-headless"] }));
    
    let c = ClientBuilder::native()
      .capabilities(caps)
      .connect("http://localhost:4444").await?;
    c.goto(link).await?;
    let html = c.source().await?;
  
    // OLD Tauri GET methods
    // let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    // let client = ClientBuilder::new()
    //   .max_redirections(3)
    //   .build()?;
    // let request = HttpRequestBuilder::new("GET", link)?
    //   .header(USER_AGENT, user_agent)?
    //   .response_type(ResponseType::Text);
    
    // // Sends the request and gets the data
    // let res = client.send(request).await?;
    // Ok(res.read().await?.data)
    Ok(html)
}

pub async fn parse_covers(data: String) -> Result<Vec<Value>, Box<dyn Error>> {
    // Get box will all items
    let fragment = Html::parse_fragment(&data);
    let item_selector = Selector::parse("div.grid.gap-2.two-col")?;
    let items = fragment.select(&item_selector).next().unwrap();

    // Get the names
    let name_selector = Selector::parse("a.title > span")?;
    let names: Vec<_> = items
      .select(&name_selector)
      .map(|e| e.text().collect::<String>())
      .collect();
    // println!("{:?}", names);

    // Get the href (for the chapters page)
    let href_selector = Selector::parse("a.title")?;
    let hrefs: Vec<_> = items
      .select(&href_selector)
      .map(|e| e.value().attr("href").unwrap_or_default())
      .collect();
    println!("{:?}", hrefs);

    // get the img links
    let img_selector = Selector::parse("img.rounded.shadow-md.w-full.h-auto")?;
    let imgs: Vec<_> = items
      .select(&img_selector)
      .map(|e| String::from(e.value().attr("src").unwrap_or_default()))
      .collect();
    // println!("{:?}", imgs);

    let mut result: Vec<Value> = Vec::new();
    for i in 0..names.len() {
      result.push(json!({
        "name": names.get(i).unwrap(),
        "img": imgs.get(i).unwrap(),
        "href": hrefs.get(i).unwrap()
      }))
    }
    Ok(result)
}

// todo!("Need to go through all pages");
pub async fn get_chapters(link: &str) -> Result<String, Box<dyn Error>> {
  println!("got here");
  let c = CLIENT.get_or_init(async {
    let client = prep_client().await.unwrap();
    client
  }).await;
  c.goto(link).await?;
  let html = c
    .find(fantoccini::Locator::Css("div.rounded.flex.flex-col.gap-2")).await?
    .html(false).await?;
  println!("{html}");
  
  Ok(html)
}
*/