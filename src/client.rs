use reqwest::blocking::Client as HttpClient;

pub struct Client {
    pub(crate) client_base_url: String,
    pub(crate) client: HttpClient,
    pub(crate) api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Result<Self, reqwest::Error> {
        let client = HttpClient::builder().build()?;
        Ok(Self {
            client_base_url: "https://gapi.svc.krunker.io/api".to_string(),
            client,
            api_key: api_key.into(),
        })
    }
}
