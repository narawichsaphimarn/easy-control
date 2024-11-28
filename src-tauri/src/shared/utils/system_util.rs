use std::{
    env,
    process::{exit, Command},
};

use hostname;

pub struct SystemUtil;

impl SystemUtil {
    pub fn get_hostname() -> Option<String> {
        match hostname::get() {
            Ok(name) => {
                // log::debug!("Hostname: {}", name.to_string_lossy());
                Some(name.to_string_lossy().into_owned())
            }
            Err(_) => {
                // log::debug!("Failed to get hostname: {}", e);
                None
            }
        }
    }

    pub fn restart_app() {
        // Get the path to the current executable
        if let Ok(current_exe) = env::current_exe() {
            // Spawn a new process of the current app
            Command::new(current_exe)
                .spawn()
                .expect("Failed to restart application");

            // Exit the current instance
            exit(0);
        } else {
            eprintln!("Failed to get the current executable path.");
        }
    }
}
