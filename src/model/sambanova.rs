use super::base::{define_provider_models, Model, Provider};

#[derive(Debug)]
pub struct SamboNova;

impl Provider for SamboNova {
    type ModelType = SamboNovaModel;

    fn provider_name() -> &'static str {
        "sambanova"
    }
}

define_provider_models!(SamboNova, SamboNovaModel, {
    (DeepSeekV3_0324, "DeepSeek-V3-0324"),
    (DeepSeekR1, "DeepSeek-R1")
});
