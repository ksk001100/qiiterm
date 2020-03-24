use reqwest::{self, blocking::Client, header, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct QiitaClient {
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trend {
    pub isNewArrival: bool,
    pub hasCodeBlock: bool,
    pub node: Node,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub createdAt: String,
    pub likesCount: usize,
    pub title: String,
    pub uuid: String,
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

    pub fn my_items(&self) -> Result<(), Error> {
        let url = "https://qiita.com/api/v2/authenticated_user/items";
        self.get(url)?;

        Ok(())
    }

    pub fn trends(&self) -> Result<Vec<Trend>, Error> {
        let url = "https://qiita-api.netlify.com/.netlify/functions/trend";
        let trends = self.get::<Vec<Trend>>(url)?;

        Ok(trends)
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
