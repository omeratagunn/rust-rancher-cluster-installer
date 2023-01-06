use crate::types::{ClusterBuild, ClusterBuilder};
use crate::utils::spinner;
use crate::yaml::parse_yaml_config;

pub fn app(path: &String, should_delete: &bool) {
    let spinner_handle = spinner("Parsing yaml file...".parse().expect("spinner working"));

    let build = ClusterBuilder {
        config: parse_yaml_config(path),
    };

    spinner_handle.done();

    if *should_delete {
        let delete = build.delete();
        match delete {
            Ok(msg) => println!("{:?}", &msg),
            Err(err) => println!("{:?}", &err),
        }
        return;
    }

    let install = build.build();

    match install {
        Ok(msg) => println!("{:?}", &msg),
        Err(err) => println!("{:?}", &err),
    }
}
