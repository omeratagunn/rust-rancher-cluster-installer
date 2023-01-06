use crate::types::{App, ClusterBuild, ClusterBuilder};
use crate::yaml::parse_yaml_config;

pub fn app(config: &App) {

    let build = ClusterBuilder {
        config: parse_yaml_config(&config.config),
    };

    if config.delete {
        let delete = build.delete();
        match delete {
            Ok(msg) => println!("{:?}", &msg),
            Err(err) => println!("{:?}", &err),
        }
        return;
    }
    if config.install {
        let install = build.build();

        match install {
            Ok(msg) => println!("{:?}", &msg),
            Err(err) => println!("{:?}", &err),
        }
    }

}
