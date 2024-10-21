use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, ClientSecret, RefreshToken, UserToken};

pub async fn get_user_token(access_token: &str, refresh_token: &str) -> UserToken {
    let client: HelixClient<reqwest::Client> = HelixClient::default();
    let client_secret = ClientSecret::new(
        env!(
            "TWITCH_CLIENT_SECRET",
            "TWITCH_CLIENT_SECRET not set at build time"
        )
        .to_string(),
    );
    let access_token = AccessToken::new(String::from(access_token));
    let refresh_token = RefreshToken::new(String::from(refresh_token));

    let token = UserToken::from_existing(&client, access_token, refresh_token, client_secret).await;
    println!("UserToken: {:#?}", token);
    let token = token.unwrap(); // TODO: Handle this
    return token;
}
