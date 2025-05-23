fn main(){
    // if the config file exists then skips this part
    // otherwise it creates the file and path
    /*
    #[cfg(not(debug_assertions))]
    let config_path = match env::var("XDG_CONFIG_HOME"){
        Ok(k) => k,
        // if XDG_CONFIG_HOME is not set then $HOME/.config should be taken
        Err(_) => 
            match env::var("HOME") {
                Ok(mut k) => {k.push_str("/.config"); k},
                Err(_) => panic!("neither XDG_CONFIG_HOME and HOME where found!"),
            }
    } + "project_manager";
    */
    let config_path = "project_manager";
    let config_data = std::fs::read_to_string(config_path).unwrap_or_else(|_|{
        if std::fs::File::create(config_path).is_err() {
            println!("could not create config file at {}", config_path);
        }
        String::new()
    });

    println!("data: {}", config_data);
}
