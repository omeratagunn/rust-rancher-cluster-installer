pub struct Installation {
    pub(crate) linux_amd64: Vec<LinuxInstructions>,
}

pub struct LinuxInstructions {
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) fallback_command: String,
}





