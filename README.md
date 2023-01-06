# !Under-development!

# Purpose

Creating your own k3s clusters requires sets of installations in order to get things running. Even though rancher simplified it a lot, for some purpose you may find this tiny application comforting( in a manner of instruction package)

## What this does?

On a given yaml configuration file ( can be seen in example folder ) it will connect to your servers via ssh and install neccessary linux packages. Create HA cluster and initiate nodes bound to master. Will prepare your machine for longhorn and provide you kubeConfig and token alltogether by creating folder where you executed this app.

### Example yaml file

```
// first master will be the one who sets the token.
// since port for ssh might differ based on your taste, you must pass port to look at.
masters:
  - name: "main"
    ip: "127.0.0.1:22"
    username: "root"
    password : "root"
    k3s_version: "v1.23.13+k3s1"
nodes:
  - name: "node1"
    ip: "127.0.0.2:22"
    username: "root"
    password : "root"
    k3s_version: "v1.23.13+k3s1"
  - name: "node2"
    ip: ""
    username: "root"
    password: "something"
    k3s_version: "v1.23.13+k3s1"
  - name: "node3"
    ip: ""
    username: "root"
    password: "something"
    k3s_version: "v1.23.13+k3s1"
```

# Example usage
```
// will iniate master in given master server, get the token and kubeconfig into the kubeconfig folder where this executable sits.
./rancherinstaller --config <path_to_yaml> --install

# if repo is cloned, then;
cargo run -- --config <path_to_yaml> --install

// if you want to delete the installation you can run
./rancherinstaller --config <path_to_yaml> --delete

# if repo is cloned, then;
cargo run -- --config <path_to_yaml> --delete

```

```
- If you are clonning the repo, you can ask for help by running `cargo run -- --help` 
- If you somehow got compiled executable, then ./rancherinstaller --help will do the same.
```
