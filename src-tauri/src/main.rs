// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use std::{collections::HashMap, os::windows::process::{self, CommandExt} , sync::Mutex};

struct AppState{
    current_dir: Mutex<String>,
}

fn create_hash() -> HashMap<String, String> {
  let mut map: HashMap<String, String> = HashMap::new();
  map.insert("ls".to_string(), "dir".to_string()); // List directory contents
  map.insert("cat".to_string(), "type".to_string()); // Display file contents
  map.insert("mv".to_string(), "move".to_string()); // Move/rename files
  map.insert("cp".to_string(), "copy".to_string()); // Copy files
  map.insert("rm".to_string(), "del".to_string()); // Remove files
  map.insert("echo".to_string(), "echo".to_string()); // Display a line of text
  map.insert("grep".to_string(), "findstr".to_string()); // Search for text in files
  map.insert("pwd".to_string(), "cd".to_string()); // Print working directory (use "cd" with no arguments)
  map.insert("mkdir".to_string(), "mkdir".to_string()); // Create a directory
  map.insert("rmdir".to_string(), "rmdir".to_string()); // Remove a directory
  map.insert("clear".to_string(), "cls".to_string()); // Clear the terminal screen
  map.insert("df".to_string(), "wmic logicaldisk get size,freespace,caption".to_string()); // Disk space usage
  map.insert("du".to_string(), "du".to_string()); // Disk usage of files and directories
  map.insert("ps".to_string(), "tasklist".to_string()); // Display currently running processes
  map.insert("kill".to_string(), "taskkill".to_string()); // Terminate processes
  map.insert("ifconfig".to_string(), "ipconfig".to_string()); // Network configuration
  map.insert("ping".to_string(), "ping".to_string()); // Send ICMP ECHO_REQUEST packets to network hosts
  map.insert("netstat".to_string(), "netstat".to_string()); // Network statistics
  map.insert("top".to_string(), "tasklist".to_string()); // Display tasks (use "tasklist" to show processes)
  map.insert("man".to_string(), "help".to_string()); // Display manual pages
  map.insert("whoami".to_string(), "whoami".to_string()); // Display the current user
  map.insert("touch".to_string(), "echo >".to_string());
  map
}


#[tauri::command]
fn execute_command(command: String) -> Result<String ,String> {
    let map = create_hash();
    let mut parts = command.split_whitespace();
    let main_command = parts.next().unwrap_or("");
    let sub_command = parts.collect::<Vec<&str>>().join(" ");
    let mapped_command: String;
  match main_command {
    "info" =>{return Ok("This is the Project in Rust By Krishna Bansal".to_owned());},
    "cls" =>{ return Ok("clean".to_owned())},
    "cd" =>{
        let new_dir = sub_command.trim();
        let error = std::env::set_current_dir(new_dir);
        if error.is_err(){
            return Err("Directory not found".to_owned());
        }
        let mut current_dir = std::env::current_dir().unwrap();
        let path = current_dir.to_str().unwrap();
        let appState = AppState{
            current_dir: Mutex::new(path.to_string()),
        };
        return Ok("".to_owned());
    },
    "exit" =>{ std::process::exit(0);},
    value=> { 
        if(map.get(value).is_none()) {
            mapped_command = command;
        }
        else{
            mapped_command = map.get(value).unwrap().to_string() + " " + &sub_command;
            
        }}
  }
  let mut current_dir = std::env::current_dir().unwrap();
    // return Ok(mapped_command.to_string());
    let output = std::process::Command::new("cmd")
        .current_dir(current_dir)
        .creation_flags(0x08000000)
        .args(&[ "/C",&mapped_command])
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn return_path() -> Result<String, String> {
    let mut current_dir = std::env::current_dir().unwrap();
    let path = current_dir.to_str().unwrap();
    let appState = AppState{
        current_dir: Mutex::new(path.to_string()),
    };
    return Ok(path.to_string());
}

fn main() {
  tauri::Builder::default()
    .manage(AppState{
        current_dir: Mutex::new("".to_string()),
    })
    .invoke_handler(tauri::generate_handler![execute_command , return_path]) 
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
