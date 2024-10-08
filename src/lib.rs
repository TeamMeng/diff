mod cli;
mod config;
mod req;
mod utils;

pub use cli::{Action, Args, RunArgs};
pub use config::{DiffConfig, DiffProfile, ResponseProfile};
pub use req::RequestProfile;
pub use utils::{diff_text, highlight_test};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}
