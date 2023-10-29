pub enum Feature {
    BasicTextGen,
}

pub struct BufNeeded {
    num_chars: usize
}

pub mod error {
    pub enum GetName {}
    pub enum GetVersion {}
    pub enum GetFeatures {}

    pub enum InstalledCheck {}
    pub enum EnabledCheck {}

    pub enum GenerationOutcome {
        ReachedBufLimitWarning,
    }

    pub enum Install {}
    pub enum Uninstall {}
    pub enum Enablement {}
    pub enum Disablement {}

    pub enum GetHyperparam {
        BufTooSmall(super::BufNeeded),
    }
}

pub enum Hyperparam {
    Temperature,
}

pub trait Ident {
    fn get_name(&self, buf: &mut String) -> Result<(), error::GetName>;
}

pub trait API {
    fn get_api_version(&self) -> Result<i32, error::GetVersion>;
}

pub trait Installation {
    fn is_installed() -> Result<bool, error::InstalledCheck>;
    fn install() -> Result<(), error::Install>;
    fn uninstall() -> Result<(), error::Uninstall>;
}

pub trait Features {
    fn get_features(&self, features: &mut Vec<Feature>) -> Result<(), error::GetFeatures>;
}

pub trait Enablement {
    fn is_enabled() -> Result<bool, error::EnabledCheck>;
    fn enable() -> Result<(), error::Enablement>;
    fn disable() -> Result<(), error::Disablement>;
}

pub trait TextGeneration {
    fn generate_text(&self, buf: &mut String) -> Result<(), error::GenerationOutcome>;
    fn get_hyperparam(&self, param: &Hyperparam, default: &String, buf: &mut String) -> Result<(), error::GetHyperparam>;
}

pub trait TextGenBackend :
    Ident
    + API
    + Installation
    + Features
    + Enablement
    + TextGeneration
{
}
