pub struct Client {
    http: reqwest::Client,
    email: String,
    api_key: String,
    // TODO: Investigate keys across servers
    server: String,
}

type HResult<T> = reqwest::Result<T>;

impl Client {
    pub fn new(email: &str, api_key: &str, server: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            email: email.to_owned(),
            api_key: api_key.to_owned(),
            server: format!("http://{}/api/v1", server),
        }
    }

    /// Send a message
    ///
    /// [Zulip docs](https://zulip.com/api/send-message)
    pub async fn send(&self, content: &str) -> HResult<String> {
        self.http
            .post(&format!("{}/messages", self.server))
            .basic_auth(&self.email, Some(&self.api_key))
            .form(&[
                ("type", "private"),
                // AARON id
                ("to", "user6@zulipdev.com"),
                ("topic", "First Post"),
                ("content", content),
            ])
            .send()
            .await?
            .text()
            .await
    }

    pub async fn users(&self) -> HResult<String> {
        self.http
            .get(&format!("{}/users", self.server))
            .basic_auth(&self.email, Some(&self.api_key))
            .send()
            .await?
            .text()
            .await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
