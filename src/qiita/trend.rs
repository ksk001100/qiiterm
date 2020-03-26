use crate::qiita::QiitaClient;
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trend {
    pub isNewArrival: bool,
    pub hasCodeBlock: bool,
    pub node: Node,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub createdAt: String,
    pub likesCount: usize,
    pub title: String,
    pub uuid: String,
    pub author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    profileImageUrl: String,
    urlName: String,
}

impl Trend {
    pub fn set_body(&mut self) {
        let md = reqwest::blocking::get(&format!(
            "https://qiita.com/{}/items/{}.md",
            &self.node.author.urlName, &self.node.uuid
        ))
        .unwrap()
        .text()
        .unwrap();

        self.body = md;
    }
}

impl QiitaClient {
    pub fn trends(&self) -> Result<Vec<Trend>, Error> {
        let url = "https://qiita-api.netlify.com/.netlify/functions/trend";
        let mut trends = self.get::<Vec<Trend>>(url)?;
        for mut trend in &mut trends {
            trend.set_body();
        }

        Ok(trends)
    }
}
