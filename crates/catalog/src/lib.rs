use spec::{Dep, Deployment, Manifest};

mod berachain;
mod ethereum;
mod polygon;

pub use berachain::BerachainDeployment;
pub use ethereum::EthereumDeployment;
pub use polygon::PolygonDeployment;

pub fn apply(dep: Dep) -> eyre::Result<Manifest> {
    match dep.module.as_str() {
        "ethereum" => EthereumDeployment::default().apply(&dep),
        "polygon" => PolygonDeployment::default().apply(&dep),
        "berachain" => BerachainDeployment::default().apply(&dep),
        _ => Err(eyre::eyre!("Unknown module: {}", dep.module)),
    }
}
