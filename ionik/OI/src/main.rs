/* 
 notes critical user :
 - You may not change, modify or even put this script into your personal project without written permission from the script creator (official author) 
 
 Telegarm    : @UnixeID | Chenel : @Yeye_PID
 Github      : Betrix-ID
 Consultasi  : betrix322@gmail.com
 
 the date this script was written : 11 - Mei - 2025
*/

use std::{
    process::Command,
    thread::sleep,
    time::Duration,
    fs::File,
    io::{BufRead, BufReader},
};

fn shell(message: &str) {
    let command = format!(
        "am start -a android.intent.action.MAIN \
         -e toasttext \"{}\" -n bellavita.toast/.MainActivity > /dev/null 2>&1",
        message
    );
    
    if let Err(e) = Command::new("sh").arg("-c").arg(command).status() {
        eprintln!("Failed to execute shell command: {}", e);
    }
}

fn adjust_ionice_high(pid: &str) {
    let cmds = [
        format!("ionice -c 2 -n 0 -p {}", pid),
        "cmd device_config put runtime_native_boot iorap_perfetto_enable true".to_string(),
        "cmd device_config put runtime_native_boot iorap_readahead_enable true".to_string(),
    ];

    for cmd in cmds {
        if let Err(e) = Command::new("sh").arg("-c").arg(&cmd).status() {
            eprintln!("Failed to execute: '{}'\nError: {}", cmd, e);
        }
    }
}

fn adjust_ionice_low(pid: &str) {
    let cmds = [
        format!("ionice -c 3 -p {}", pid),
        "cmd device_config put runtime_native_boot iorap_perfetto_enable true".to_string(),
        "cmd device_config put runtime_native_boot iorap_readahead_enable true".to_string(),
    ];

    for cmd in cmds {
        if let Err(e) = Command::new("sh").arg("-c").arg(&cmd).status() {
            eprintln!("Failed to execute: '{}'\nError: {}", cmd, e);
        }
    }
}

fn ps_proses(proses_name: &str) {
    let shell_cmd = format!(
        r#"ps | grep "{}" | grep system | awk '{{print $9}}'"#,
        proses_name
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(shell_cmd)
        .output()
        .expect("Gagal menjalankan ps pipeline");

    let Ok(list_text) = String::from_utf8(output.stdout) else {
        eprintln!("Gagal decode output");
        return;
    };

    for line in list_text.lines() {
        let process_name = line.trim();
        if process_name.is_empty() {
            continue;
        }

        let pgrep_cmd = format!("pgrep -f {}", process_name);
        let pgrep_output = Command::new("sh")
            .arg("-c")
            .arg(pgrep_cmd)
            .output();

        if let Ok(output) = pgrep_output {
            if let Ok(pid_list) = String::from_utf8(output.stdout) {
                for pid in pid_list.lines() {
                    let pid = pid.trim();
                    if pid.is_empty() {
                        continue;
                    }

                    let ionice_cmd = format!("ionice -c 2 -n 0 -p {}", pid);
                    let _ = Command::new("sh")
                        .arg("-c")
                        .arg(ionice_cmd)
                        .status();
                }
            }
        }
    }
}

fn check_app_running(app_name: &str) -> bool {
    let command = format!(
        "dumpsys SurfaceFlinger | grep Output | head -n 1 | cut -f1 -d/ | awk -F '(' '{{print $2}}' | grep -w \"{}\"",
        app_name
    );

    let output = Command::new("sh").arg("-c").arg(command).output();
    match output {
        Ok(out) => !String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        Err(_) => false,
    }
}

fn get_pid_from_name(name: &str) -> Option<String> {
    let command = format!("pgrep -f {}", name);
    let output = Command::new("sh").arg("-c").arg(command).output().ok()?;
    let result = String::from_utf8_lossy(&output.stdout).lines().next()?.trim().to_string();
    Some(result)
}


fn main() {
    shell("♨️ Multicor Priority is running in the background");
    sleep(Duration::from_secs(1));
    shell("♨️ Multicor Priority game: By @UnixeID");

    let primary_path = "/sdcard/data/GamePid.txt";
    let mut app_in_running = String::new();

    loop {
        let file = File::open(primary_path);
        if file.is_err() {
            eprintln!("gamelist.txt tidak ditemukan.");
            sleep(Duration::from_secs(2));
            continue;
        }

        let reader = BufReader::new(file.unwrap());
        let mut app_found = false;

        for line in reader.lines().flatten() {
            let app_name = line.trim();
            if app_name.is_empty() {
                continue;
            }

            if check_app_running(app_name) {
                app_found = true;

                if app_in_running != app_name {
                    shell(&format!("♨️ Multicor Priority High : {}", app_name));
                    app_in_running = app_name.to_string();

                    // Set priority HIGH
                    if let Some(pid) = get_pid_from_name(app_name) {
                        adjust_ionice_high(&pid);
                        ps_proses(&app_name);
                    }
                }

                break; 
            }
        }

        if !app_found {
            if !app_in_running.is_empty() {
                shell("♨️ Multicor Priority Low : game close");

                if let Some(pid) = get_pid_from_name(&app_in_running) {
                    adjust_ionice_low(&pid);
                }

                app_in_running.clear();
            }
        }
        
        sleep(Duration::from_secs(2));
    }
}