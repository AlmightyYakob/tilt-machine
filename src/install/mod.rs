use std::{
    env,
    fs::{self, File},
    io::Write,
};

const SERVICE_FILE_LOCATION: &str = "/etc/systemd/system/tilt-machine-service";

fn generate_service_file() {
    let cur_exe = env::current_exe().expect("Could not retrieve self exec path.");
    let mut file = File::create(SERVICE_FILE_LOCATION)
        .expect("Could not create system file. Try running as sudo/root.");

    let command = format!("ExecStart={} run\n", cur_exe.display());
    let service_file_lines = [
        "[Unit]\n",
        "Description=Start the Monitor Rotation Service\n\n",
        "[Service]\n",
        command.as_str(),
        "Type=oneshot\n",
    ];

    for line in service_file_lines.iter() {
        file.write(line.as_bytes())
            .expect("Could not write to service file.");
    }
}

fn start_and_enable_service() {
    // TODO
}

fn remove_service_file() {
    fs::remove_file(SERVICE_FILE_LOCATION).expect("Could not remove service file.")
}

fn stop_and_disable_service() {
    // TODO
}

pub fn install() {
    generate_service_file()
}

pub fn uninstall() {
    remove_service_file()
}
