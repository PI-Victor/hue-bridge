use hue_bridge::HueBridge;
use hue_bridge::Light;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //NOTE: this action requires the user to push the button on the hue bridge
    //beforehand
    let url = env::var("HUE_BRIDGE_URL")?;
    let pem_path = env::var("HUE_BRIDGE_PEM_PATH")?;

    let client = HueBridge::builder()
        .api(&url)
        .ca_pem(&pem_path)
        .app_name("hue-bindings")
        .build()
        .await
        .map_err(|e| format!("failed to register app: {}", e))?;

    // NOTE: save the username generated here for future use
    println!(
        "app: {}, generated username: {}",
        client.get_app_name(),
        client.get_username()
    );

    Ok(())
}
