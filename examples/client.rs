use hue_bridge::HueBridge;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HueBridge::new("https:://my-hue.local", "my-app");
    Ok(())
}
