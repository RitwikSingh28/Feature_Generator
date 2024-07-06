use std::path::PathBuf;

use clap::Arg;
use create_dirs::create_directories;

mod create_dirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::command!()
        .author("Ritwik Singh itsritwik26@gmail.com")
        .name("FeatureFolderizer")
        .arg(
            Arg::new("feature_name")
            .short('f')
            .long("feature")
            .value_name("FEATURE_NAME")
            .required(true)
            .help("The name of the feature to be created")
        )
        .arg(
            Arg::new("base_dir")
            .short('b')
            .long("base")
            .value_name("BASE_DIR")
            .help("The base directory of the Flutter project (default: current directory)")
        )
        .get_matches();

    let feature_name = matches.get_one::<String>("feature_name").unwrap();
    
    let mut curr_dir = if let Some(base_dir) = matches.get_one::<String>("base_dir") {
        PathBuf::from(base_dir)
    } else {
        std::env::current_dir()?
    };

    curr_dir.push("lib");

    let path_exists = curr_dir.try_exists()?;
    if !path_exists {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound, "Current directory is not root of the Flutter Project")
        ));
    }

    println!("Creating directories for feature: {}", feature_name);
    let _ = create_directories(&mut curr_dir, &feature_name)?;
    println!("Directory structure created successfully");

    Ok(())
}
