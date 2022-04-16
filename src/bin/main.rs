use std::env;

use wingman;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut wingman = wingman::Wingman::new().await?;

    // if let Some(user) = env::args().skip(1).next() {
    //     println!("User: {} live status: {:?})", user, wingman.is_live(&user).await?);
    // }

    wingman.run().await?;

    panic!("No user name provided as an argumebt!");
}