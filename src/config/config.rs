pub struct Installation {
    pub(crate) linux_amd64: Vec<LinuxInstructions>,
}

pub struct LinuxInstructions {
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) fallback_command: String,
    pub(crate) target_node: String
}

pub(crate) fn get_installation() -> Installation {
    Installation {
        linux_amd64: Vec::from([
            LinuxInstructions {
                name: "curl".to_string(),
                command: "curl --version".to_string(),
                fallback_command: "apt install curl -y".to_string(),
                target_node: "any".to_string()
            },
            LinuxInstructions {
                name: "jq".to_string(),
                command: "jq --version".to_string(),
                fallback_command: "apt-get update && apt-get install jq -y".to_string(),
                target_node: "any".to_string()
            },
            LinuxInstructions {
                name: "nfs common".to_string(),
                command: "apt update && apt install mount nfs-common -y || true && apt install -y open-iscsi".to_string(),
                fallback_command: "apt update && apt install mount nfs-common -y || true && apt install -y open-iscsi".to_string(),
                target_node: "any".to_string()
            },
            LinuxInstructions{
                name: "Longhorn check".to_string(),
                command: "https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash".to_string(),
                fallback_command: "".to_string(),
                target_node: "master".to_string()
            },
            LinuxInstructions {
                name: "Longhorn check".to_string(),
                command: "curl -sSfL https://raw.githubusercontent.com/longhorn/longhorn/master/scripts/environment_check.sh | bash".to_string(),
                fallback_command: "curl -sSfL https://raw.githubusercontent.com/longhorn/longhorn/master/scripts/environment_check.sh | bash".to_string(),
                target_node: "any".to_string()
            }])
    }
}




