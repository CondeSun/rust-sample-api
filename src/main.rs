#[tokio::mai]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rust_proj_sample::run().await?;
}
