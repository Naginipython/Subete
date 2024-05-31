use serde_json::Value;

mod mangadex;

#[tauri::command]
pub async fn search_manga(name: String) -> Vec<Value> {
  // Creates the GET request
  let data = match mangadex::get_covers(&name).await {
      Ok(d) => d,
      Err(_e) => Value::default()
  };
  println!("{data}");

  // Parses the JSON
  let result = match mangadex::parse_covers(data) {
    Ok(r) => r,
    Err(_e) => Vec::new()
  };

  // result
  vec![Value::default()]
}

#[tauri::command]
pub async fn get_chapters(href: String) -> Vec<String> {
    // let link = format!("https://mangadex.org{}", href);
    // let html = match mangadex::get_chapters(&link).await {
    //     Ok(d) => d,
    //     Err(_e) => String::default(),
    // };
    // println!("{html}");
    // todo!("find chapters (both paired and not paired)");

    vec!["yeah".to_string()]
}