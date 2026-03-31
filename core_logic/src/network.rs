use reqwest::Result;

pub async fn fetch_wttr_data() -> Result<String> {
    reqwest::get("https://wttr.in?format=j1")
        .await?
        .text()
        .await
}
