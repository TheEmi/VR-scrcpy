// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use async_process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Manager;
use std::io::{self, BufRead, BufReader};
use tokio::time::{timeout, Duration};
#[cfg(target_os = "windows")]
use tokio::process::Command as TokioCommand;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn launch_app(adress: String, process: String) -> Result<String, String>{
    println!("{} {}", adress, process);
    let mut cmd = TokioCommand::new("adb");
    cmd.arg("-s")
    .arg(adress)
    .arg("shell")
    .arg("am")
    .arg("start")
    .arg("-n")
    .arg(process);
        #[cfg(target_os = "windows")]
        {
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
    let output = cmd.output()
    .await;
    return Ok("App launched".to_string())
}

async fn close_app(adress: String, process: String) -> Result<String, String>{
    let mut cmd = TokioCommand::new("adb");
    cmd.arg("-s")
    .arg(adress)
    .arg("shell")
    .arg("am")
    .arg("force-stop")
    .arg(process);
#[cfg(target_os = "windows")]
{
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
}
    let output = cmd.output()
    .await;
    return Ok("App launched".to_string())
}

#[tauri::command]
async fn start_server(var: String) -> Result<String, String>{
    let output = Command::new("adb")
    .arg("start-server")
    .output()
    .await;
    return Ok("App launched".to_string())
}

#[tauri::command]
async fn stop_server(var: String) -> Result<String, String>{
    let output = Command::new("adb")
    .arg("kill-server")
    .output()
    .await;
    return Ok("App launched".to_string())
}



#[tauri::command]
async fn scrcpy_device(adress: String) -> Result<String, String>{
    let together = format!("--window-title={adress}");
    let mut cmd = TokioCommand::new("scrcpy");
    cmd.arg("-s")
    .arg(adress)
    .arg("-b5M")
    .arg("--crop")
    .arg("1730:974:1880:450")
    .arg("--max-fps")
    .arg("30")
    .arg("--no-audio")
    .arg(together);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    let output = cmd.output();
    return Ok("Scrcpy launched".to_string())
}

async fn connect_to_device(address: String) -> Result<String, String>{
    // Set a timeout duration of 3 seconds
    let duration = Duration::from_secs(3);
    let mut cmd = TokioCommand::new("adb");
    cmd.arg("connect")
        .arg(&address);

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    
    // Run the command with a timeout
    let command_future = cmd.output();
    
    // Run the command with a timeout
    match timeout(duration, command_future).await {
        Ok(Ok(output)) => {
            if output.status.success() {
                Ok(format!("Connected to {}", address))
            } else {
                Err(format!("Failed to connect to {}: {:?}", address, output))
            }
        }
        Ok(Err(e)) => Err(format!("Failed to run command for {}: {}", address, e)),
        Err(_) => Err(format!("Command timed out for {}", address)),
    }
}

async fn disconnect_device(address: String) -> Result<String, String>{
    // Create the command
    let mut cmd = TokioCommand::new("adb");
    cmd.arg("disconnect")
    .arg(&address);
    #[cfg(target_os = "windows")]
   {
       cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
   }
    cmd.output();
    Ok("Disconnected".to_string())
}

#[tauri::command]
async fn disconnect_multiple_devices(addresses: Vec<String>) -> Result<String, String>{
    let mut handles = vec![];
    for address in addresses {
        let handle = tokio::spawn(async move{
            let result = disconnect_device(address).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    return Ok("All apps launched".to_string())
}

#[tauri::command]
async fn scrcpy_device_multiple(addresses: Vec<String>) -> Result<String, String>{
    let mut handles = vec![];
    for address in addresses {
        let handle = tokio::spawn(async move{
            let result = scrcpy_device(address).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    return Ok("All apps launched".to_string())
}

#[tauri::command]
async fn connect_to_ips() -> Result<String, String>{
    let mut handles = vec![];
    let file_path = "./src/ip_list.txt";
    let mut file = OpenOptions::new()   
    .read(true)
    .open(file_path)
    .expect("Failed to open file");
// Create a BufReader for efficient reading
let reader = BufReader::new(file);

// Initialize a vector to store lines
let mut lines = Vec::new();

// Read the file line by line
for line in reader.lines() {
        // Unwrap the Result and push the line to the vector
        lines.push(line);
    }
    for address in lines {
        
        let handle = tokio::spawn(async move{
            let result = connect_to_device(address.unwrap()).await;
            match result {
                Ok(message) => println!("Success: {}", message),
                Err(e) => eprintln!("Error: {}", e),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    Ok("All devices connected".to_string())
}

#[tauri::command]
async fn launch_app_multiple(addresses: Vec<String>, process: String) -> Result<String, String> {
    let mut handles = vec![];
    for address in addresses {
        let process_clone = process.clone();
        let handle = tokio::spawn(async move{
            let result = launch_app(address, process_clone).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    return Ok("All apps launched".to_string())
}

#[tauri::command]
async fn close_app_multiple(addresses: Vec<String>, process: String) -> Result<String, String> {
    let mut handles = vec![];
    for address in addresses {
        let process_clone = process.clone();
        let handle = tokio::spawn(async move{
            let result = close_app(address, process_clone).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    return Ok("All apps closed".to_string())
}

#[tauri::command]
async fn get_connected_devices() -> Vec<String> {
    let mut cmd = TokioCommand::new("adb");
    cmd.arg("devices");
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
        let output = cmd.output()
            .await;
    return match output {
        Ok(output) => {
            let mut outputs = vec![];
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if(line == "" || line.contains("List of devices attached") || line.contains("daemon not running") || line.contains("daemon started successfully") || line.contains("adb server is out of date") || line.contains("daemon not running; starting now at tcp:5037") || line.contains("daemon started successfully") || line.contains("daemon not running; starting now at tcp:5037")) {
                    continue;
                }
                outputs.push(String::from(line));
                
                println!("{} end of line", line);
            }
            return outputs;
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
            let mut errors = vec![];
            errors.push(String::from("Error running command"));
            return errors;
        }
    };
}

#[tauri::command]
fn insert_new_ip(ip: &str) {
    let file_path = "./src/ip_list.txt";
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    // Write a newline character to the end of the file
    match writeln!(file, "{}", ip) {
        Ok(_) => println!("New line inserted successfully"),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}

#[tauri::command]
fn insert_new_process(appName: &str) {
    let file_path = "./src/processes.txt";
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    // Write a newline character to the end of the file
    match writeln!(file, "{}", appName) {
        Ok(_) => println!("New line inserted successfully"),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}

#[tauri::command]
fn get_processes() -> Vec<String> {
    let file_path = "./src/processes.txt";
    let mut file = OpenOptions::new()   
    .read(true)
    .open(file_path)
    .expect("Failed to open file");
            let reader = BufReader::new(file);
            let mut lines = Vec::new();
            for line in reader.lines() {
                lines.push(line.unwrap());
            }
            return lines;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![insert_new_ip, get_connected_devices, launch_app, start_server, stop_server, disconnect_multiple_devices, scrcpy_device,scrcpy_device_multiple, launch_app_multiple, close_app_multiple, connect_to_ips,insert_new_process, get_processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
