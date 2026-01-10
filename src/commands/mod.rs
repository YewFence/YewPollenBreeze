mod alias;
mod apply;
mod check;
mod clean;
pub mod config;
pub mod hook;
mod push;
mod status;

pub use alias::execute as alias;
pub use apply::execute as apply;
pub use check::execute as check;
pub use clean::execute as clean;
pub use config::execute as config;
pub use push::execute as push;
pub use status::execute as status;
