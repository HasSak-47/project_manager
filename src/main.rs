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
    let mut config_data = std::fs::read_to_string(config_path).unwrap_or_else(|_|{
        if std::fs::File::create(config_path).is_err() {
            println!("could not create config file at {}", config_path);
        }
        String::new()
    });

    while config_data.find("\n\n").is_some(){
        config_data = config_data.replace("\n\n", "\n");
    }
    let lines_split = config_data.split('\n');
    let mut lines_raw = Vec::<String>::new();
    for line in lines_split{
        if line.len() == 0{
            continue;
        }
        lines_raw.push(line.to_string());
    }

    println!("{:?}", lines_raw);
    // test push
}
