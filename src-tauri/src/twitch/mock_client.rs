pub struct Wrapper<T> {
    client: T,
    replace_host: String,
}

impl Wrapper<reqwest::Client> {
    pub fn new(client: reqwest::Client, replace_host: String) -> Self {
        Wrapper {
            client,
            replace_host,
        }
    }
}

impl<C> twitch_api::HttpClient for Wrapper<C>
where
    C: twitch_api::HttpClient,
{
    type Error = C::Error;

    fn req(
        &self,
        request: twitch_api::client::Request,
    ) -> twitch_api::client::BoxedFuture<
        '_,
        Result<twitch_api::client::Response, <Self as twitch_api::HttpClient>::Error>,
    > {
        let (mut parts, rest) = request.into_parts();
        parts.uri = format!(
            "{}{}",
            self.replace_host,
            parts
                .uri
                .path_and_query()
                .expect("a path should be set")
                .as_str()
                .replace("helix/", "")
        )
        .parse()
        .unwrap();
        println!("req: {:?}", parts.uri);
        self.client
            .req(twitch_api::client::Request::from_parts(parts, rest))
    }
}
