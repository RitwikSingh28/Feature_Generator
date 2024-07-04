use std::path::PathBuf;

use create_dirs::{create_data_dir, create_domain_dir, create_presentation_dir};
mod create_dirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String>= std::env::args().collect();

    let feature_name = &args[1];

    let mut curr_dir = std::env::current_dir()?;
    curr_dir.push("lib");

    let path_exists = curr_dir.try_exists()?;
    if !path_exists {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Current directory is not root of the Flutter Project")));
    }

    curr_dir.push("feature");
    curr_dir.push(PathBuf::from(feature_name));

    create_data_dir(&mut curr_dir)?;
    create_domain_dir(&mut curr_dir)?;
    create_presentation_dir(&mut curr_dir)?;

    return Ok(());
}
