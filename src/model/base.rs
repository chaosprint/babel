pub trait Model {
    fn model_id(&self) -> &'static str;
}

pub trait Provider {
    type ModelType: Model;
    fn provider_name() -> &'static str;
}

macro_rules! define_provider_models {
    ($provider:ident, $enum_name:ident, {
        $(($variant:ident, $value:expr)),*
    }) => {
        // Define model enum
        pub enum $enum_name {
            $($variant),*
        }
        
        // Implement Model trait for enum
        impl Model for $enum_name {
            fn model_id(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value),*
                }
            }
        }
    }
}

pub(crate) use define_provider_models;