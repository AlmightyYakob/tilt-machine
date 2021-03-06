use std::{
    env,
    fs::{self, File},
    io::Write,
    process::Command,
};

const SERVICE_FILE_LOCATION: &str = "/etc/systemd/system/tilt-machine.service";

fn generate_service_file() {
    let cur_exe = env::current_exe().expect("Could not retrieve self exec path.");
    let mut file = File::create(SERVICE_FILE_LOCATION)
        .expect("Could not create system file. Try running as sudo/root.");

    let command = format!("ExecStart={} run\n", cur_exe.display());
    let service_file_lines = [
        "[Unit]\n",
        "Description=The Monitor Rotation Service\n\n",
        "[Service]\n",
        command.as_str(),
        "Type=simple\n\n",
        "[Install]\n",
        "WantedBy=multi-user.target\n",
    ];

    for line in service_file_lines.iter() {
        file.write(line.as_bytes())
            .expect("Could not write to service file.");
    }
}

fn start_and_enable_service() {
    Command::new("systemctl")
        .args(&["enable", "tilt-machine.service"])
        .output()
        .expect("Failed to enable tilt-machine service");
    Command::new("systemctl")
        .args(&["start", "tilt-machine"])
        .output()
        .expect("Failed to start tilt-machine service");

    println!("Service successfully started and enabled!");
}

fn remove_service_file() {
    fs::remove_file(SERVICE_FILE_LOCATION).expect("Could not remove service file")
}

fn stop_and_disable_service() {
    Command::new("systemctl")
        .args(&["stop", "tilt-machine"])
        .output()
        .expect("Failed to stop tilt-machine service");
    Command::new("systemctl")
        .args(&["disable", "tilt-machine.service"])
        .output()
        .expect("Failed to disable tilt-machine service");

    println!("Service successfully stopped and disabled!");
}

pub fn install() {
    generate_service_file();
    start_and_enable_service();
}

pub fn uninstall() {
    stop_and_disable_service();
    remove_service_file();
}
