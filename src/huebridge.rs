use reqwest::Client;

pub struct HueBridge<S: AsRef<str>> {
    client: Client,
    api_url: S,
    app_name: S,
}

impl<S> HueBridge<S>
where
    S: AsRef<str>,
{
    pub fn new(api_url: S, app_name: S) -> Self {
        let client = Client::new();
        Self {
            client,
            api_url,
            app_name,
        }
    }
}
