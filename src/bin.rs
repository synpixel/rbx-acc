mod api;
mod auth;
mod cli;
mod save;
mod utils;

use clap::Parser;
use cli::RbxAcc;
use std::process;

#[tokio::main]
async fn main() {
    let rbx_acc = RbxAcc::parse();

    match rbx_acc.run().await {
        Ok(()) => {}
        Err(err) => {
            process::exit(err.exit_code());
        }
    }
}
