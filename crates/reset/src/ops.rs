use std::io::Write;

/// The reset sequence:
/// - \x1bc       - RIS (Reset to Initial State)
/// - \x1b[0m     - SGR reset (turn off all attributes)
/// - \x1b[2J     - Clear entire screen
/// - \x1b[H      - Move cursor to home position (1,1)
pub const RESET_SEQUENCE: &[u8] = b"\x1bc\x1b[0m\x1b[2J\x1b[H";

pub fn reset_terminal(output: &mut dyn Write) -> std::io::Result<()> {
    output.write_all(RESET_SEQUENCE)?;
    output.flush()
}
