use clap::Parser;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InteractiveMode {
    #[default]
    Never,
    Once,
    Always,
}

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "rm",
    about = "Remove (unlink) the FILE(s).",
    version,
    disable_help_flag = true
)]
pub struct RmConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Ignore nonexistent files and arguments, never prompt
    #[arg(short = 'f', long = "force")]
    pub force_flag: bool,

    /// Prompt before every removal
    #[arg(short = 'i')]
    pub interactive_always: bool,

    /// Prompt once before removing more than three files or when removing recursively
    #[arg(short = 'I')]
    pub interactive_once: bool,

    /// Prompt according to WHEN: never, once (-I), or always (-i)
    #[arg(long = "interactive", value_name = "WHEN", num_args = 0..=1, default_missing_value = "always", require_equals = true)]
    pub interactive_when: Option<String>,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', short_alias = 'R', long = "recursive")]
    pub recursive: bool,

    /// Remove empty directories
    #[arg(short = 'd', long = "dir")]
    pub dir: bool,

    /// Explain what is being done
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Do not treat '/' specially
    #[arg(long = "no-preserve-root")]
    pub no_preserve_root: bool,

    /// Do not remove '/' (default); with 'all', reject any command line argument on a separate device
    #[arg(long = "preserve-root", value_name = "all")]
    pub preserve_root_flag: Option<Option<String>>,

    /// Files to remove
    #[arg()]
    pub paths: Vec<String>,

    #[arg(skip)]
    pub force: bool,

    #[arg(skip)]
    pub interactive: InteractiveMode,

    #[arg(skip = true)]
    pub preserve_root: bool,

    #[arg(skip)]
    pub preserve_root_all: bool,
}

impl Default for RmConfig {
    fn default() -> Self {
        Self {
            help: None,
            force_flag: false,
            interactive_always: false,
            interactive_once: false,
            interactive_when: None,
            recursive: false,
            dir: false,
            verbose: false,
            no_preserve_root: false,
            preserve_root_flag: None,
            paths: Vec::new(),
            force: false,
            interactive: InteractiveMode::Never,
            preserve_root: true,
            preserve_root_all: false,
        }
    }
}

impl RmConfig {
    /// Resolve computed fields from the raw clap flags.
    /// Must be called after parsing.
    pub fn resolve(mut self) -> Result<Self, String> {
        // Default preserve_root to true
        self.preserve_root = true;

        if self.no_preserve_root {
            self.preserve_root = false;
        }

        if let Some(ref val) = self.preserve_root_flag {
            self.preserve_root = true;
            if let Some(s) = val
                && s == "all"
            {
                self.preserve_root_all = true;
            }
        }

        // Resolve interactive mode.
        // The last-specified flag wins, but with clap we process in order:
        // --interactive=WHEN takes precedence if set, otherwise check -f, -i, -I.
        if let Some(ref when) = self.interactive_when {
            match when.as_str() {
                "never" => {
                    self.interactive = InteractiveMode::Never;
                }
                "once" => {
                    self.interactive = InteractiveMode::Once;
                    self.force = false;
                }
                "always" => {
                    self.interactive = InteractiveMode::Always;
                    self.force = false;
                }
                _ => {
                    return Err(format!(
                        "rm: invalid argument '{}' for '--interactive'\nValid arguments are: 'never', 'once', 'always'",
                        when
                    ));
                }
            }
        } else if self.force_flag {
            self.force = true;
            self.interactive = InteractiveMode::Never;
        } else if self.interactive_always {
            self.interactive = InteractiveMode::Always;
            self.force = false;
        } else if self.interactive_once {
            self.interactive = InteractiveMode::Once;
            self.force = false;
        }

        Ok(self)
    }
}
