use std::path::PathBuf;

pub fn create_directories(base: &mut PathBuf, feature_name: &str) -> Result<(), std::io::Error> {
    base.push(PathBuf::from(feature_name));

    create_data_dir(base)?;
    create_domain_dir(base)?;
    create_presentation_dir(base)?;

    Ok(())
}

fn create_sub_dirs(base: &mut PathBuf, subdirs: &[&str]) -> Result<(), std::io::Error> {
    for subdir in subdirs {
        base.push(subdir);
        std::fs::create_dir_all(&base)?;
        base.pop();
    }

    Ok(())
}

fn create_data_dir(base: &mut PathBuf) -> Result<(), std::io::Error> {
    base.push("data");

    let result = create_sub_dirs(base, &["datasource", "models", "repositories"]);
    base.pop();
    result
}

fn create_domain_dir(base: &mut PathBuf) -> Result<(), std::io::Error> {
    base.push("domain");

    let result = create_sub_dirs(base, &["entities", "repositories", "usecases"]);
    base.pop();
    result
}

fn create_presentation_dir(base: &mut PathBuf) -> Result<(), std::io::Error> {
    base.push("presentation");

    let result = create_sub_dirs(base, &["bloc", "pages", "widgets"]);
    base.pop();

    result
}


