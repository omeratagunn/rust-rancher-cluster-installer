use crate::types::{
    App, ClusterBuild, ClusterBuilder, K3s, LinuxInstructions, OsInstallationSequence,
};
use crate::yaml::parse_yaml_config;

pub fn app(env_input: &App) {
    let build = ClusterBuilder {
        config: parse_yaml_config(&env_input.config),
        installation: OsInstallationSequence {
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
        },
        kube_type: K3s{}
    };

    if env_input.delete {
        let delete = build.delete();
        match delete {
            Ok(msg) => println!("{:?}", &msg),
            Err(err) => println!("{:?}", &err),
        }
        return;
    }
    if env_input.install {
        let install = build.build();

        match install {
            Ok(msg) => println!("{:?}", &msg),
            Err(err) => println!("{:?}", &err),
        }
    }
}
