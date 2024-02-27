use std::fs::File;

fn main() {

    let file_path = std::env::args().nth(1).unwrap_or("".to_string());

    if file_path == "" {
        println!("Error: No file paths provided!");
        std::process::exit(1);
    }

    let file_paths: Vec<&str> = file_path.split(",").collect();

    let mut errors: Vec<String> = vec![];
    let mut warnings: Vec<String> = vec![];

    file_paths.iter().for_each(|file_path| {
        check_files(file_path, &mut errors, &mut warnings);
    });

    errors.iter().for_each(|error| {
        println!("ERROR: {}", error);
    });

    warnings.iter().for_each(|warning| {
        println!("WARNING: {}", warning);
    });

    if errors.len() > 0 {
        std::process::exit(1);
    }

}

fn check_files(file_path: &str, errors: &mut Vec<String>, warnings: &mut Vec<String>) {

    let env_file_name = ".env";
    let env_example_file_name = ".env.example";

    let env_file_path = format!("{}/{}", file_path, env_file_name);
    let env_example_file_path = format!("{}/{}", file_path, env_example_file_name);

    let env_file = File::open(&env_file_path);

    if env_file.is_err() {
        errors.push(format!("Missing .env file at {}", file_path));
    }

    let env_example_file = File::open(&env_example_file_path);

    if env_example_file.is_err() {
        warnings.push(format!("Missing .env.example file at {}", file_path));
    }

    if env_file.is_ok() && env_example_file.is_ok() {
        let env_file = env_file.unwrap();
        let env_example_file = env_example_file.unwrap();

        let env_file_metadata = env_file.metadata().unwrap();
        let env_example_file_metadata = env_example_file.metadata().unwrap();

        if env_file_metadata.len() == 0 {
            errors.push(format!("Empty .env file at {}", file_path));
        }

        if env_example_file_metadata.len() == 0 {
            warnings.push(format!("Empty .env.example file at {}", file_path));
        }

        if env_file_metadata.len() > 0 && env_example_file_metadata.len() > 0 {

            // Check for missing keys (ERRORS)

            let env_file = std::fs::read_to_string(env_file_path).unwrap();
            let env_example_file = std::fs::read_to_string(env_example_file_path).unwrap();

            let env_file_lines: Vec<&str> = env_file.split("\n").collect();
            let env_example_file_lines: Vec<&str> = env_example_file.split("\n").collect();

            let mut missing_keys: Vec<String> = vec![];

            env_example_file_lines.iter().for_each(|env_example_file_line| {
                if !env_example_file_line.is_empty() {
                    if env_example_file_line.starts_with("#") {
                        return;
                    }

                    let key = env_example_file_line.split("=").collect::<Vec<&str>>()[0];
                    let mut found = false;

                    env_file_lines.iter().for_each(|env_file_line| {
                        if !env_file_line.is_empty() {
                            let env_file_key = env_file_line.split("=").collect::<Vec<&str>>()[0];
                            if key == env_file_key {
                                found = true;
                            }
                        }
                    });

                    if !found {
                        missing_keys.push(key.to_string());
                    }
                }
            });

            if missing_keys.len() > 0 {
                errors.push(format!("{} missing from {}",
                                    missing_keys.join(", "),
                                    env_file_name));
            }

            // Check for unused keys (WARNINGS)

            let mut unused_keys: Vec<String> = vec![];

            env_file_lines.iter().for_each(|env_file_line| {
                if !env_file_line.is_empty() {
                    if env_file_line.starts_with("#") {
                        return;
                    }

                    let key = env_file_line.split("=").collect::<Vec<&str>>()[0];
                    let mut found = false;

                    env_example_file_lines.iter().for_each(|env_example_file_line| {
                        if !env_example_file_line.is_empty() {
                            let env_example_file_key = env_example_file_line.split("=").collect::<Vec<&str>>()[0];
                            if key == env_example_file_key {
                                found = true;
                            }
                        }
                    });

                    if !found {
                        unused_keys.push(key.to_string());
                    }
                }
            });

            if unused_keys.len() > 0 {
                warnings.push(format!("{} unused in {}",
                                      unused_keys.join(", "),
                                      env_file_name));
            }
        }
    }
}
