use hue_bridge::HueBridge;
use hue_bridge::Light;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("HUE_BRIDGE_URL")?;
    let user_name = env::var("HUE_BRIDGE_REGISTERED_USERNAME")?;
    let pem_path = env::var("HUE_BRIDGE_PEM_PATH")?;

    let client = HueBridge::builder()
        .api(&url)
        .ca_pem(&pem_path)
        .token(&user_name)
        .app_name("hue-bindings")
        .build()
        .await?;

    println!(
        "app: {}, generated username: {}",
        client.get_app_name(),
        client.get_username()
    );

    let light = Light::new(client);
    let lights = light.list().await?;
    lights.iter().for_each(|light| println!("{:?}", &light));

    let light = light.get("235eb2fc-98de-4462-850b-f255d9a39995").await?;
    println!("{:?}", &light);

    Ok(())
}
