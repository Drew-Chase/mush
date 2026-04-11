use std::io;

use crate::cli::HostnameConfig;

pub fn get_hostname(config: &HostnameConfig) -> io::Result<String> {
    let hostname = gethostname::gethostname()
        .into_string()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "hostname is not valid UTF-8"))?;

    if config.short {
        Ok(hostname.split('.').next().unwrap_or(&hostname).to_string())
    } else {
        Ok(hostname)
    }
}
