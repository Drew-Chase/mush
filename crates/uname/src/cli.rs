use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "uname", about = "Print certain system information", version, disable_help_flag = true)]
pub struct UnameConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'a', long = "all", help = "Print all information")]
    pub all: bool,

    #[arg(short = 's', long = "kernel-name", help = "Print the kernel name")]
    pub kernel_name: bool,

    #[arg(short = 'n', long = "nodename", help = "Print the network node hostname")]
    pub nodename: bool,

    #[arg(short = 'r', long = "kernel-release", help = "Print the kernel release")]
    pub kernel_release: bool,

    #[arg(short = 'v', long = "kernel-version", help = "Print the kernel version")]
    pub kernel_version: bool,

    #[arg(short = 'm', long = "machine", help = "Print the machine hardware name")]
    pub machine: bool,

    #[arg(short = 'p', long = "processor", help = "Print the processor type")]
    pub processor: bool,

    #[arg(short = 'o', long = "operating-system", help = "Print the operating system")]
    pub operating_system: bool,
}

impl UnameConfig {
    pub fn resolve(&mut self) {
        if self.all {
            self.kernel_name = true;
            self.nodename = true;
            self.kernel_release = true;
            self.kernel_version = true;
            self.machine = true;
            self.processor = true;
            self.operating_system = true;
        }

        if !self.kernel_name
            && !self.nodename
            && !self.kernel_release
            && !self.kernel_version
            && !self.machine
            && !self.processor
            && !self.operating_system
        {
            self.kernel_name = true;
        }
    }
}
