///show info
pub fn show_info() -> anyhow::Result<()> {
    println!("CONFIG_PATH_ENV_KEY: {}", crate::domain::constants::CONFIG_PATH_ENV_KEY);
    println!(
        "ENV {} VALUE: {:?}",
        crate::domain::constants::CONFIG_PATH_ENV_KEY,
        std::env::var(crate::domain::constants::CONFIG_PATH_ENV_KEY)
    );
    println!("DEFAULT FILE NAME: {}", crate::domain::constants::DEFAULT_FILE_NAME);
    println!("DEFAULT CONFIG CONTENT:\n{}", crate::domain::constants::DEFAULT_CONFIG);

    Ok(())
}
