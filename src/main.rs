use hyper::{header, Client, Method, Request};
use hyper_tls::HttpsConnector;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static URI_WEBHOOK: &str = "https://outlook.office.com/webhook/ec2f0739-7f10-4697-939d-631d93736de5@c6014ab3-8365-4c93-bcd9-972ca2a738b4/IncomingWebhook/f0b9efc40d034ecc8708a9122cfb88cf/881b8a36-96a2-4bab-b37b-b291ad7274cc";
static SUCCESS_DATA: &str = r#"{ summary: "장애여부 체크", "sections": [ { "activityTitle": "우이 홈페이지 상태 점검", "activitySubtitle": "점검결과", "activityText": "정상" } ] }"#;
static FAIL_DATA: &str = r#"{ "summary": "장애여부 체크", "sections": [ { "activityTitle": "우이 홈페이지 상태 점검", "activitySubtitle": "점검결과", "activityText": "에러발생" } ] }"#;

#[tokio::main]
async fn main() {
    let mut interval_timer = tokio::time::interval(chrono::Duration::hours(3)
        .to_std().unwrap());
    loop {
        interval_timer.tick().await;
        // For async task
        tokio::spawn(async {
            match check_url().await {
                Ok(_result) => {},
                Err(error) => {
                    println!("Error when checking the homepage status: {}", error);
                }
            }; });
        // For blocking task
        // tokio::task::spawn_blocking(|| check_url());
    }
}

async fn check_url() -> Result<()> {
    let client = Client::new();
    let uri = "http://ui-line.com".parse::<hyper::Uri>().unwrap();
    let resp = client.get(uri).await?;

    if resp.status().is_success() {
        println!("Homepage Success: {}", resp.status());
        let req = Request::builder()
            .method(Method::POST)
            .uri(URI_WEBHOOK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(SUCCESS_DATA.into())
            .unwrap();
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let res = client.request(req).await?;
        println!("Webhook Response: {}", res.status());
    } else {
        println!("Homepage Fail: {}", resp.status());
        let req = Request::builder()
            .method(Method::POST)
            .uri(URI_WEBHOOK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(FAIL_DATA.into())
            .unwrap();
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let res = client.request(req).await?;
        println!("Webhook Response: {}", res.status());
    }
    return Ok(());
}
