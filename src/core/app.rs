use rancherinstaller::types::{ClusterBuild, ClusterBuilder};

pub(crate) fn app(path: &String, should_delete: &bool) {
    let spinner_handle =
        rancherinstaller::utils::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = rancherinstaller::yaml::parse_yaml_config(path);

    spinner_handle.done();

    let build = ClusterBuilder {
        config: parsed_yaml
    };

    if *should_delete {
        let delete = build.delete();
        match delete {
            Ok(msg) => println!("{:?}", &msg),
            Err(err) => println!("{:?}", &err)
        }
        return;
    }


    let install = build.build();

    match install {
        Ok(msg) => println!("{:?}", &msg),
        Err(err)=> println!("{:?}", &err)
    }

}
