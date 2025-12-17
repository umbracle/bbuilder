use runtime_docker_compose::DockerRuntime;
use runtime_trait::Runtime;
use spec::{Dep, Manifest};
use std::{env, fs};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Get filename from command-line arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)?;
    let input: Dep = serde_json::from_str(contents.as_str())?;

    println!("input {:?}", input);

    let manifest = catalog::apply(input)?;

    let svc = Service::new(DockerRuntime::new("composer".to_string()));
    svc.deploy(manifest).await?;

    Ok(())
}

struct Service {
    runtime: DockerRuntime,
}

impl Service {
    fn new(runtime: DockerRuntime) -> Self {
        Self { runtime }
    }

    async fn deploy(&self, manifest: Manifest) -> eyre::Result<()> {
        self.runtime.run(manifest).await
    }
}
