use steam_rs::{Steam, AppId};
mod common;

const EXAMPLE_APP_ID: AppId = AppId(440); // Team Fortress 2

#[test]
pub fn get_news() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_news_for_app(EXAMPLE_APP_ID, 1, 10).await.unwrap());
    });
}