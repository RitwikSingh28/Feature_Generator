use std::path::PathBuf;

pub fn create_data_dir(base: &mut PathBuf) -> Result<(), std::io::Error> {
    base.push("data");

    base.push("datasource");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.push("models");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.push("repositories");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.pop();
    Ok(())
}

pub fn create_domain_dir(base: &mut PathBuf) -> Result<(), std::io::Error> {
    base.push("domain");

    base.push("entities");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.push("repositories");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.push("usecases");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.pop();
    Ok(())
}

pub fn create_presentation_dir(base: &mut PathBuf) -> Result<(), std::io::Error> {
    base.push("presentation");

    base.push("bloc");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.push("pages");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.push("widgets");
    std::fs::create_dir_all(&base)?;
    base.pop();

    base.pop();
    Ok(())
}
