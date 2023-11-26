#[derive(Clone, clap::Parser, Debug)]
#[cfg_attr(test, derive(Default))]
pub struct Config {
    #[clap(env)]
    pub alerts_to_email: String,
}
