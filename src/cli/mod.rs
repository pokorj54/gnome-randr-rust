use std::time::Duration;

use dbus::blocking::Connection;
use structopt::StructOpt;

use crate::display_config::DisplayConfig;

pub mod query;

use query::handle_query;

#[derive(StructOpt)]
enum Command {
    #[structopt(
        about = "Query returns information about the current state of the monitors. This is the default subcommand."
    )]
    Query(query::CommandOptions),
}

#[derive(StructOpt)]
#[structopt(
    about = "A program to query information about and manipulate displays on Gnome with Wayland.",
    long_about = "A program to query information about and manipulate displays on Gnome with Wayland.\n\nDefault command is `query`."
)]
struct CLI {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the CLI args. We do this first to short-circuit the dbus calls if there's an invalid arg.
    let args = CLI::from_args();

    // Open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Open a proxy to the Mutter DisplayConfig
    let proxy = conn.with_proxy(
        "org.gnome.Mutter.DisplayConfig",
        "/org/gnome/Mutter/DisplayConfig",
        Duration::from_millis(5000),
    );

    // Load the config from dbus using the proxy
    let config = DisplayConfig::get_current_state(&proxy)?;

    // See what we're executing
    let cmd = args
        .cmd
        .unwrap_or(Command::Query(query::CommandOptions { connector: None }));

    print!(
        "{}",
        match cmd {
            Command::Query(opts) => handle_query(&opts, &config, &proxy)?,
        }
    );

    Ok(())
}