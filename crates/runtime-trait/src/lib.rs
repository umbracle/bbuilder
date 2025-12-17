use spec::Manifest;

#[async_trait::async_trait]
pub trait Runtime {
    async fn run(&self, manifest: Manifest) -> eyre::Result<()>;
}
