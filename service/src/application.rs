use crate::server::ApplicationBuilder;

#[derive(Clone)]
pub struct AbOptimisationApplication {}

pub struct AbOptimisationApplicationBuilder {}

impl ApplicationBuilder<AbOptimisationApplication> for AbOptimisationApplicationBuilder {
    fn build(self) -> anyhow::Result<AbOptimisationApplication> {
        Ok(AbOptimisationApplication {})
    }
}