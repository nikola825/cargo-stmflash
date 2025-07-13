use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    // 1. Get the project metadata, extract the name and the project name, and the name of the binary compiled for thumbv7em-none-eabihf target
    // 2. Find the location of stm32 cube programmer from STM32_CUBE_PROGRAMMER env var
    // 3. Invoke cargo objcopy to get the ihex file
    // 4. Invoke stm32 cube programmer to flash the hex file

    let project = cargo_project::Project::query(".").expect("Not in a valid cargo project dir");
    let artifact = cargo_project::Artifact::Bin(project.name());
    println!("Flashing project {}", project.name());

    let path = project
        .path(
            artifact,
            cargo_project::Profile::Release,
            Some("thumbv7em-none-eabihf"),
            "x86_64-unknown-linux-gnu",
        )
        .expect("Couldn't find the build result");

    println!("ELF path {:?}", path);
    let bin_path = format!("./{}.hex", project.name());
    println!("Binary path {bin_path}");

    let stm_programmer_path = env::var("STM32_CUBE_PROGRAMMER")
        .expect("STM32_CUBE_PROGRAMMER not set, need it to find cube programmer");

    let objcopy_status = Command::new("cargo")
        .arg("objcopy")
        .args(["--release", "--", "-O", "ihex", &bin_path])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("Objcopy exited with {}", objcopy_status);
    if !objcopy_status.success() {
        panic!("Objcopy failed to complete");
    }

    let flash_status = Command::new(stm_programmer_path)
        .args(["-c", "port=usb1", "-d", &bin_path, "-g"]) // Assume usb1 for DFU, -g means reset after flashing
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("Flash exited with {}", flash_status);
    if !objcopy_status.success() {
        panic!("Flash failed to complete");
    }
}
