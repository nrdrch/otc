use std::env;
use std::io;
use std::path::PathBuf;
use std::process::{Command, exit};
use strum_macros::{Display, EnumIter};
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use colored::*;
fn checkfs() {
    let user_profile = env::var("USERPROFILE").unwrap();
    let ompthemes = PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes", user_profile));

    if !ompthemes.exists() {
        std::fs::create_dir_all(&ompthemes).expect("Failed to create Custom OMPTheme Dir.");
    }
    let user_profile = env::var("USERPROFILE").unwrap();
    let _flag_path = PathBuf::from(format!("{}/Documents/WindowsPowerShell/omp_is_installed_flag.txt", user_profile));
}


fn check_oh_my_posh_installed() {
    let user_profile = env::var("USERPROFILE").unwrap();
    let flag_path = PathBuf::from(format!("{}/Documents/WindowsPowerShell/omp_is_installed_flag.txt", user_profile));
    let _destination_file = PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes/custom.omp.json", user_profile));
    let _ompthemes = PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes", user_profile));

    if flag_path.exists() {
        return;
    } else {
        let output = Command::new("powershell.exe")
            .arg("oh-my-posh")
            .output()
            .unwrap_or_else(|_| {
                eprintln!("Failed to execute PowerShell command.");
                exit(1);
            });

        if output.status.success() {
            // Create flag file to indicate installation is complete
            File::create(&flag_path).expect("Failed to create flag file.");
            println!("{}{}{}","[".white(),"SUCCESS".green(),"]".white(),);
        } else { 
            // oh-my-posh is not installed, so install it
            let install_command = "winget install JanDeDobbeleer.OhMyPosh -s winget";
            //let install_command2 =
            //    "Set-ExecutionPolicy Bypass -Scope Process -Force; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://ohmyposh.dev/install.ps1'))";

            Command::new("powershell.exe")
                .arg("-Command")
                .arg(install_command)
                .spawn()
                .expect("Failed to execute PowerShell install command.");

            File::create(&flag_path).expect("Failed to create flag file.");
            println!("oh-my-posh is installed.");
        }
    }
}
const PACKAGENAME: &str= "otc";
#[derive(Debug, EnumIter, Display)]
enum CommandOption {
    List,
    Update,
    Choose,
    Help,
    Add,
}
fn usage() {
    let spacer = "       ";

    println!(" {}","Usage Examples:".cyan().bold());
    print!("{}",">".white().bold());
    println!("{}","                                                                              ".white().strikethrough().dimmed().bold());
    println!("{} {} {} {} {}","  <".yellow().dimmed().bold(),  PACKAGENAME.white(),"--list-themes".cyan().bold(),"or".white(),"-ls ".cyan().bold());
    println!("{}{} {}",spacer,"List the themes quickly in plain text |".white(), "for previews run Get-PoshThemes".white().dimmed());
    println!("{} {} {} {} {}","  <".yellow().dimmed().bold(),  PACKAGENAME.white(),"--update-omp".cyan().bold(),"or".white(),"-u".cyan().bold());
    println!("{}{}",spacer,"Update the themes and Oh My Posh".white());
    println!("{} {} {} {} {} {}","  <".yellow().dimmed().bold(),  PACKAGENAME.white(),"--choose-theme".cyan().bold(),"or".white(),"-ch".cyan().bold(), "<Name>".white());
    println!("{}{}",spacer,"Choose a theme by name".white());
    println!("{} {} {} {} {} {} {}","  <".yellow().dimmed().bold(),  PACKAGENAME.white(),"--add--newtheme".cyan().bold(),"or".white(),"-a".cyan().bold(), "<Name>".white(),"<Path>".white().dimmed(),);
    println!("{}{} {}",spacer,"Add a custom theme from path |".white(),"if Path not specified, copy from current theme".white().dimmed());
    println!("{} {} {} {} {}","  <".yellow().dimmed().bold(),  PACKAGENAME.white(),"-help".cyan().bold(),"or".white(),"-h".cyan().bold());
    println!("{}{} ",spacer,"Display help".white());
    print!("{}","   ".green());
}
fn get_modded_posh_themes() -> Result<Vec<PathBuf>, io::Error> {
    let user_profile = env::var("USERPROFILE").unwrap();
    let themes_dir = PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes", user_profile));
    if themes_dir.exists() && themes_dir.is_dir() {
        let mut modded_themes = Vec::new();
        for entry in std::fs::read_dir(themes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                modded_themes.push(path);
            }
        }
        Ok(modded_themes)
    } else {
        Ok(vec![])
    }
}
fn get_posh_themes() -> Result<Vec<PathBuf>, io::Error> {
    let temp_dir = env::temp_dir().join("oh-my-posh");
    let themes_dir = temp_dir.join("themes");
    if themes_dir.exists() && themes_dir.is_dir() {
        let mut themes = Vec::new();
        for entry in std::fs::read_dir(themes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                themes.push(path);
            }
        }
        Ok(themes)
    } else {
        Ok(vec![])
    }
}
fn run_command(command: &str, theme_name: Option<&str>, source_file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "--list-themes" | "-ls" => {
            let themes = get_posh_themes()?;
            let modded_themes = get_modded_posh_themes()?;
            let _all_themes: Vec<_> = themes.iter().chain(modded_themes.iter()).collect();

            let filtered_modded_themes: Vec<_> = modded_themes
                .iter()
                .filter(|theme| {
                    let theme_name = theme
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .map(|name| name.trim_end_matches(".omp"))
                        .unwrap_or("Invalid Theme Name");

                    theme_name != "custom" && !themes.contains(&theme)
                })
                .collect();
                print!("{}",">".cyan().bold());
                println!("{}","                             ".white().strikethrough().dimmed().bold());
                
                println!("{}"," Themes:".cyan().bold().bold());
            for (_index, theme) in themes.iter().enumerate() {
                let theme_name = theme
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|name| name.trim_end_matches(".omp"))
                    .unwrap_or("Invalid Theme Name");

                if theme_name != "custom" && !filtered_modded_themes.iter().any(|modded_theme| {
                    modded_theme
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .map(|name| name.trim_end_matches(".omp"))
                        .unwrap_or("Invalid Theme Name") == theme_name
                }) {
                    println!("      {}", theme_name);
                }
            }

            print!("{}",">".cyan().bold());
            println!("{}","                             ".white().strikethrough().dimmed().bold());
            
            println!("{}"," Modded dimmed:".cyan().bold().bold());
            for (_index, theme) in filtered_modded_themes.iter().enumerate() {
                let modded_theme_name = theme
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|name| name.trim_end_matches(".omp"))
                    .unwrap_or("Invalid Theme Name");

                println!("      {}", modded_theme_name);
            }

            println!("{}","     ".white().dimmed());
            
            println!("{}"," For a preview run: Get-PoshThemes.".dimmed().bold());
            Ok(())
        }
        "--update-omp" | "-u" => {
            let temp_dir = std::env::temp_dir().join("oh-my-posh");
            if temp_dir.exists() {
                std::fs::remove_dir_all(&temp_dir)?;
            }
            
            // Start the spinner
            let mut spinner = Spinner::new(Spinners::Line, "Updating Oh-My-Posh...".into());
            
            
            Command::new("powershell.exe")
                .args(&["winget", "upgrade", "JanDeDobbeleer.OhMyPosh", "-s", "winget"])
                .stdout(std::process::Stdio::null()) // Redirect stdout to null
                .stderr(std::process::Stdio::null()) // Redirect stderr to null
                .spawn()
                .expect("Failed to execute Update command.");
        
            
        
            Command::new("git")
                .args(&[
                    "clone",
                    "https://github.com/JanDeDobbeleer/oh-my-posh.git",
                    &temp_dir.to_string_lossy(),
                ])
                .stdout(std::process::Stdio::null()) // Redirect stdout to null
                .stderr(std::process::Stdio::null()) // Redirect stderr to null
                .spawn()
                .expect("Failed to execute git clone command.");
        
            // Stop the spinner
            sleep(Duration::from_secs(3));
            spinner.stop();
            
            println!("{}{}{}","[".white(),"SUCCESS".green(),"]".white(),);
            Ok(())
        }
        "-help " | "-h" => {
            usage();
            Ok(())
        }
        "--choose-theme" | "-ch" => {
            let themes = get_posh_themes()?;
            let modded_themes = get_modded_posh_themes()?;
        
            let all_themes: Vec<_> = themes.iter().chain(modded_themes.iter()).collect();
        
            let selected_theme_path = all_themes.iter().find(|theme| {
                theme
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|name| name.trim_end_matches(".omp"))
                    .unwrap_or("Invalid Theme Name") == theme_name.map(|name| name.trim_end_matches(".omp.json")).unwrap_or("Invalid Theme Name")
            });
        
            if let Some(path) = selected_theme_path {
                let user_profile = env::var("USERPROFILE").unwrap();
                let destination = PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes/custom.omp.json", user_profile));
                std::fs::copy(path, &destination)?;
                print!("    Changing Theme ");
                println!("{}{}{}","[".white(),"SUCCESS".green(),"]".white(),);
            } else {
                let modded_theme_names: Vec<_> = modded_themes
                    .iter()
                    .map(|theme| {
                        theme
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .map(|name| name.trim_end_matches(".omp"))
                            .unwrap_or("Invalid Theme Name")
                    })
                    .collect();
        
                if modded_theme_names.contains(&"custom") {
                    println!("{} {}{}{}","Failed to find the selected theme.".white(), "[".white(),"ERROR".red(),"]".white());
                } else {
                    println!("{} {}{}{}","Invalid theme name.".white(), "[".white(),"ERROR".red(),"]".white());
                }
            }
        
            Ok(())
        }
        "--add-newtheme" | "-a" => {
            let user_profile = env::var("USERPROFILE").unwrap();
            let default_destination = PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes/custom.omp.json", user_profile));
            
            if let Some(source) = source_file {
                let source = PathBuf::from(source);
                if !source.exists() {
                    println!("{} {}{}{}","Source file does not exist.".white(), "[".white(),"ERROR".red(),"]".white());
                    return Err(Box::new(io::Error::new(
                        io::ErrorKind::NotFound,
                        "Source file does not exist",
                    )));
                }
        
                let destination_file = if let Some(name) = theme_name {
                    PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes/{}.omp.json", user_profile, name))
                } else {
                    default_destination.clone()
                };
        
                fs_extra::file::copy(&source, &destination_file, &fs_extra::file::CopyOptions::new())?;
        
                print!("    Adding Theme ");
                println!("{}{}{}","[".white(),"SUCCESS".green(),"]".white(),);
                Ok(())
            } 
            else {
                let destination_file = if let Some(name) = theme_name {
                    PathBuf::from(format!("{}/Documents/WindowsPowerShell/ompthemes/{}.omp.json", user_profile, name))
                } else {
                    default_destination.clone()
                };
            
                fs_extra::file::copy(&default_destination, &destination_file, &fs_extra::file::CopyOptions::new())?;
            
                print!("    Theme added ");
                println!("{}{}{}","[".white(),"SUCCESS".green(),"]".white(),);
                Ok(())
            }
        }
        
              
        _ => {
            usage();
            Ok(())
        }
    }
}

fn main() {
    checkfs();
    check_oh_my_posh_installed();

    let args: Vec<String> = env::args().skip(1).collect();
    if let Some(command) = args.get(0) {
        let theme_name = args.get(1).map(|name| name.as_str());
        let source_file = args.get(2).map(|file| file.as_str());
        if let Err(err) = run_command(command, theme_name, source_file) {
            eprintln!("Error: {}", err);
            exit(1);
        }
    } else {
        eprintln!("{} {}{}{}","No command provided.".white(), "[".white(), "ERROR".red(),"]".white());
        print!("{} {} {}{}{}","Please run:".white().dimmed(), PACKAGENAME.white().bold(), "-help".cyan().bold(), " or ".white(), "-h".cyan().bold());
    }
}
