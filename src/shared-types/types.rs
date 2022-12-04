pub struct YamlInput {
    pub(crate)masters: Vec<ServerConfigurationArgs>,
    pub(crate)nodes: Vec<ServerConfigurationArgs>,
}
pub struct ServerConfigurationArgs{
    pub(crate)name: String,
    pub(crate)ip: String,
    pub(crate)username: String,
    pub(crate)password: String
}
