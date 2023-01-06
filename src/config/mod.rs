use crate::types::{OsInstallationSequence, LinuxInstructions};

pub fn get_installation() -> OsInstallationSequence {
    OsInstallationSequence {
        linux_amd64: Vec::from([
            LinuxInstructions {
                name: "curl".to_string(),
                command: "curl --version".to_string(),
                fallback_command: "apt install curl -y".to_string(),
            },
            LinuxInstructions {
                name: "jq".to_string(),
                command: "jq --version".to_string(),
                fallback_command: "apt-get update && apt-get install jq -y".to_string(),
            },
            LinuxInstructions {
                name: "nfs common".to_string(),
                command: "apt update && apt install mount nfs-common -y || true && apt install -y open-iscsi".to_string(),
                fallback_command: "apt update && apt install mount nfs-common -y || true && apt install -y open-iscsi".to_string(),
            },
            LinuxInstructions{
                name: "Longhorn check".to_string(),
                command: "https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash".to_string(),
                fallback_command: "".to_string(),
            },
            LinuxInstructions {
                name: "Longhorn check".to_string(),
                command: "curl -sSfL https://raw.githubusercontent.com/longhorn/longhorn/master/scripts/environment_check.sh | bash".to_string(),
                fallback_command: "curl -sSfL https://raw.githubusercontent.com/longhorn/longhorn/master/scripts/environment_check.sh | bash".to_string(),
            }])
    }
}
