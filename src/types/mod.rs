pub struct Installation {
    pub linux_amd64: Vec<LinuxInstructions>,
}

pub struct LinuxInstructions {
    pub name: String,
    pub command: String,
    pub fallback_command: String,
}
