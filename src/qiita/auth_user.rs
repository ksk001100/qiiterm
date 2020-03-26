use crate::qiita::QiitaClient;
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    // pub rendered_body: String,
    // pub body: String,
    // pub coediting: bool,
    // pub comments_count: usize,
    // pub created_at: String,
    // pub group: Group,
    // pub id: String,
    // pub likes_count: usize,
    // pub private: bool,
    // pub reactions_count: usize,
    // pub tags: Vec<Tag>,
    pub title: String,
    // pub updated_at: String,
    pub url: String,
    // pub user: User,
    // pub page_views_count: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: usize,
    pub created_at: String,
    pub name: String,
    pub private: bool,
    pub updated_at: String,
    pub url_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub versions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub description: String,
    pub facebook_id: Option<String>,
    pub followees_count: usize,
    pub followers_count: usize,
    pub github_login_name: String,
    pub id: String,
    pub items_count: String,
    pub linkedin_id: String,
    pub location: String,
    pub name: String,
    pub organization: String,
    pub permanent_id: usize,
    pub profile_image_url: String,
    pub team_only: bool,
    pub twitter_screen_name: String,
    pub website_url: String,
}

impl QiitaClient {
    pub fn auth_items(&self) -> Result<Vec<Item>, Error> {
        let url = "https://qiita.com/api/v2/authenticated_user/items?per_page=100";
        let items = self.get::<Vec<Item>>(url)?;

        Ok(items)
    }
}
