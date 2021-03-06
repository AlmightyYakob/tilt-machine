use clap::{App, AppSettings, Arg, SubCommand};

mod install;
mod run;

fn main() {
    let app = App::new("Tilt Machine")
        .version("0.1.0")
        .author("Jacob Nesbit <jjnesbitt2@gmail.com>")
        .about(
            "Communicates with an Arduino to detect monitor rotation and adjust screen orientation",
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("The xrandr output that this program should operate on"),
        )
        .subcommand(SubCommand::with_name("install").about("Installs the Tilt Detection Service"))
        .subcommand(
            SubCommand::with_name("uninstall").about("Uninstalls the Tilt Detection Service"),
        )
        .subcommand(SubCommand::with_name("run").about("Runs the Tilt Detection Program"))
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let matches = app.get_matches();
    if let Some(_matches) = matches.subcommand_matches("install") {
        install::install();
    } else if let Some(_matches) = matches.subcommand_matches("uninstall") {
        install::uninstall();
    } else if let Some(_matches) = matches.subcommand_matches("run") {
        run::run();
    }
}
