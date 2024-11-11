use hostname;

pub struct SystemUtil;

impl SystemUtil {
    pub fn get_hostname() -> Option<String> {
        match hostname::get() {
            Ok(name) => {
                log::debug!("Hostname: {}", name.to_string_lossy());
                Some(name.to_string_lossy().into_owned())
            }
            Err(e) => {
                log::debug!("Failed to get hostname: {}", e);
                None
            }
        }
    }
}
