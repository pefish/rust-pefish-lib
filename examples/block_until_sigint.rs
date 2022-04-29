
use anyhow::{Context, Result, Ok};
use block_until_sigint::block;

#[tokio::main]
async fn main() -> Result<()> {

    block(async {
        println!("Hello, world!");
    }).await?;
    
    Ok(())
}
