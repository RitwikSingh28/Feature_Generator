use create_dirs::create_directories;

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

    let _ = create_directories(&mut curr_dir, &feature_name)?;

    Ok(())
}
