use super::base::{define_provider_models, Model, Provider};

#[derive(Debug)]
pub struct SambaNova;

impl Provider for SambaNova {
    type ModelType = SambaNovaModel;

    fn provider_name() -> &'static str {
        "sambanova"
    }
}

define_provider_models!(SambaNova, SambaNovaModel, {
    (DeepSeekV3_0324, "DeepSeek-V3-0324"),
    (DeepSeekR1, "DeepSeek-R1")
});
