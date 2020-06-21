extern crate dialoguer;
use std::process::{Command, Stdio};
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {    
    let cmd_line_selections = &[
         "/usr/bin/scrcpy"
        ,"/usr/bin/scrcpy -b 1M"
        ,"/usr/bin/scrcpy -b 1M --max-size 800"
        ,"/usr/bin/scrcpy -b 1M --max-size 800 --max-fps 10"
        ,"Exit Application"
    ];
    // print!("\x1B[2J\x1B[1;1H"); //This will clear the screen.
    //
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\x1B[2J\x1B[1;1H Please select a command:")
        .default(0)
        .items(&cmd_line_selections[..])
        .interact_opt()
        .unwrap();
    //
    if let Some(selection) = selection {
        // println!("Selected command: {}!", cmd_line_selections[selection]);
        if cmd_line_selections[selection] == "Exit Application" { 
            std::process::exit(0);
        } 
        else {
            // os = Linux
            // let output = Command::new("sh")
            Command::new("sh")
            .arg("-c")
            .arg(cmd_line_selections[selection])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() // .output()
            .expect("failed to execute process");
            //
            // println!("status: {}", output.status);
            // io::stdout().write_all(&output.stdout).unwrap();
            // io::stderr().write_all(&output.stderr).unwrap();
            // assert!(output.status.success());
            //
            //if cfg!(target_os = "windows") {
            //    let output = Command::new("cmd")
            //            .args(&["/C", "echo hello"])
            //            .output()
            //            .expect("failed to execute process");                         
        } 

        let adb_command_selections = &[
             "Home Button"
            ,"Open Notifications"
            ,"Tasks Button"
            ,"Exit Application"
        ];
        //
        while let Some(selected_adb_cmd) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\x1B[2J\x1B[1;1H Please select a command:")
        .default(0)
        .items(&adb_command_selections[..])
        .interact_opt()
        .unwrap()  
        {
            if adb_command_selections[selected_adb_cmd] == "Exit Application" {
                break; // println!("Selected command: {}!", adb_command_selections[selected_adb_cmd]);                
            } else {
                if adb_command_selections[selected_adb_cmd] == "Home Button" {                   
                    // os = Linux
                    Command::new("sh")
                    .arg("-c")
                    .arg("/usr/bin/adb shell input keyevent 3")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("failed to execute process");                    
                    //
                    // if cfg!(target_os = "windows") {
                    //   let output = Command::new("cmd")
                    //            .args(&["/C", "echo hello"])
                    //            .output()
                    //            .expect("failed to execute process");                    
                } else if adb_command_selections[selected_adb_cmd] == "Open Notifications" {                   
                    Command::new("sh")
                    .arg("-c")
                    .arg("/usr/bin/adb shell input swipe 10 10 10 1000")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("failed to execute process");                                     
                } else if adb_command_selections[selected_adb_cmd] == "Tasks Button" {                   
                    Command::new("sh")
                    .arg("-c")
                    .arg("/usr/bin/adb shell input keyevent 82")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("failed to execute process");               
                }
                //
                continue;
            } 
        }
    } else {        
        std::process::exit(0); // println!("You didn't select any command!");
    }
}
