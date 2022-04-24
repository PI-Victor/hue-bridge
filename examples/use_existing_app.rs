use hue_bridge::HueBridge;
use hue_bridge::Light;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("HUE_BRIDGE_URL")?;
    let user_name = env::var("HUE_BRIDGE_REGISTERED_USERNAME")?;
    let pem_path = env::var("HUE_BRIDGE_PEM_PATH")?;

    let client = HueBridge::new(&url)
        .with_ca_pem(&pem_path)
        .await?
        .with_username(&user_name)
        .await?;

    println!(
        "app: {}, generated username: {}",
        client.get_app_name(),
        client.get_username()
    );

    let light = Light::new(client);
    let lights = light.list().await?;
    lights.iter().for_each(|light| println!("{:?}", &light));

    Ok(())
}
