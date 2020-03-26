use reqwest::{self, blocking::Client, header, Error};
use serde::Deserialize;

pub mod auth_user;
pub mod trend;

pub struct QiitaClient {
    client: Client,
}

impl QiitaClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(api_key).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build();

        match client {
            Ok(client) => Self { client },
            Err(_) => Self {
                client: Client::new(),
            },
        }
    }

    fn get<'a, T: ?Sized>(&self, url: &str) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de> + 'a,
    {
        let resp = self.client.get(url).send()?;
        let v: T = resp.json()?;

        Ok(v)
    }
}
