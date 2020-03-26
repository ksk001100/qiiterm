use crate::qiita::QiitaClient;
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};

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
    pub fn trends(&self) -> Result<Vec<Trend>, Error> {
        let url = "https://qiita-api.netlify.com/.netlify/functions/trend";
        let trends = self.get::<Vec<Trend>>(url)?;

        Ok(trends)
    }
}
