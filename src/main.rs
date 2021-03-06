mod run;
mod install;

fn main() {
    // run::run().expect("Error running service");
    install::install();
    // install::uninstall();
}
