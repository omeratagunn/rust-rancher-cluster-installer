
pub struct ServerConfigurationArgs{
    pub(crate)name: String,
    pub(crate)ip: String,
    pub(crate)username: String,
    pub(crate)password: String
}

pub struct SshCredArgs{
    pub(crate) cred: ServerConfigurationArgs
}
