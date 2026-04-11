use std::io;

pub fn current_username() -> io::Result<String> {
    #[cfg(windows)]
    {
        std::env::var("USERNAME")
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "USERNAME not set"))
    }

    #[cfg(not(windows))]
    {
        std::env::var("USER")
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "USER not set"))
    }
}
