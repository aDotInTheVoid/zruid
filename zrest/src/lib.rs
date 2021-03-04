use serde::{Deserialize, Serialize};

mod errors;

pub struct Client {
    http: reqwest::Client,
    email: String,
    api_key: String,
    // TODO: Investigate keys across servers
    server: String,
}

type HResult<T> = reqwest::Result<T>;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(druid, derive(druid::Data, druid::Lens))]
pub struct Streams {
    pub streams: Vec<Stream>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(druid, derive(druid::Data, druid::Lens))]
/// <https://zulip.com/api/get-streams>
pub struct Stream {
    pub stream_id: u64,
    pub name: String,
    pub description: String,
    pub date_created: u64,
    pub is_web_public: bool,
    pub invite_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(druid, derive(druid::Data, druid::Lens))]
pub struct Topic {
    pub max_id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(druid, derive(druid::Data, druid::Lens))]
pub struct Topics {
    pub topics: Vec<Topic>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(druid, derive(druid::Data, druid::Lens))]
pub struct Messages {
    pub found_newest: bool,
    pub found_oldest: bool,
    pub found_anchor: bool,
    pub history_limited: bool,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub avatar_url: Option<String>,
    pub client: String,
    pub content: String,
    pub id: u64,
    pub is_me_message: bool,
    pub reactions: Vec<Reaction>,
    pub recipient_id: u64,
    pub sender_email: String,
    pub sender_full_name: String,
    pub sender_id: u64,
    pub sender_realm_str: String,
    pub stream_id: Option<u64>,
    #[serde(rename = "subject")]
    pub topic: String,
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub flags: Flags,
    pub last_edit_timestamp: Option<u64>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Stream,
    Private,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    // TODO https://zulip.com/api/get-messages
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flags(
    // TODO https://zulip.com/api/get-messages
    Vec<String>,
);

impl Client {
    pub fn new(email: &str, api_key: &str, server: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            email: email.to_owned(),
            api_key: api_key.to_owned(),
            server: format!("{}/api/v1", server),
        }
    }

    // TODO: Add these again with types

    /// Send a message
    ///
    /// [Zulip docs](https://zulip.com/api/send-message)

    // pub async fn send(&self, content: &str) -> HResult<String> {
    //     self.post("messages")
    //         .form(&[
    //             ("type", "private"),
    //             // AARON id
    //             ("to", "user6@zulipdev.com"),
    //             ("topic", "First Post"),
    //             ("content", content),
    //         ])
    //         .send()
    //         .await?
    //         .text()
    //         .await
    // }

    // Maily to debug for getting ids
    // pub async fn users(&self) -> HResult<String> {
    //     self.get("users").send().await?.text().await
    // }

    pub async fn streams(&self) -> HResult<Streams> {
        self.get("streams").send().await?.json().await
    }

    pub async fn topics(&self, stream_id: u64) -> HResult<Topics> {
        self.get(&format!(
            "users/me/{stream_id}/topics",
            stream_id = stream_id
        ))
        .send()
        .await?
        .json()
        .await
    }

    pub async fn messages(&self) -> HResult<Messages> {
        // https://zulip.com/api/get-messages
        #[derive(Debug, Serialize)]
        struct Params {
            num_before: u64,
            num_after: u64,
            // See https://github.com/nox/serde_urlencoded/issues/66: Nicer things blocked on
            // Longstantding serde issue. May be able to get arround with custom ser impl
            anchor: String,
            apply_markdown: bool,
        }

        let params = Params {
            num_after: 0,
            num_before: 5000,
            anchor: "newest".into(),
            // Keep as html
            apply_markdown: false,
        };

        self.get("messages")
            .query(&params)
            .send()
            .await?
            // TODO: Have pattern to deserialize json error returned.
            .json()
            .await
    }

    fn request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        self.http
            .request(method, &format!("{}/{}", self.server, url))
            .basic_auth(&self.email, Some(&self.api_key))
    }

    fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::GET, url)
    }

    fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::POST, url)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
