use native_dialog::{FileDialog, MessageDialog,MessageType};
use std::env::set_current_dir;
use std::path::Path;
use std::fs;
use std::env;
use std::process::Command;
//struct pub_dialog{
   // command_file_name:String;
//}
pub fn pub_dialog(command_name_clone: &str){
    let path = FileDialog::new()
                        .set_location("~/Downloads")
                        .add_filter("MP3 Audio", &["mp3"])
                        .show_open_single_file()
                        .unwrap();
                    let path = match path{
                        Some(path) => path,
                        None => return,
                    };
                    let yes = MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Select File To Make new Command")
                        .set_text(&format!("use {:#?}", path))
                        .show_confirm()
                        .unwrap();
                    if yes{
                       
                        //todo pathing from selected file path to default /src/sounds folder and rename file to &mut command_name
                        let mut mp3_path = Path::new(&path);
                        
                        let mut command_str = command_name_clone.to_string();
                       // let current_dir = std::env::current_dir();
                        let mut command_file_name = command_str + ".mp3";
                        let mut command_file_name_str = &command_file_name;
                        
                        //let mut command_file_name_str = command_file_name;
                    
//let mut command_file_name_str =  command_file_name;
                       let mut target_dir = "/eframe_test/sound_fx";
                       let mut target_dir_full = target_dir.to_owned() + command_file_name_str;
                      // let mut target_dir = "src\\sound_fx\\";
                      
                      
                      let mut target_dir_path = Path::new(&target_dir_full); 

                        //let mut target_crafted = self.target_dir += command_file_name;
                        Command::new("move")
                            .arg(mp3_path)
                            .arg(&target_dir_path);
                    
                      //std::env::set_current_dir("/eframe_test/").expect("Could Not Find Directory");
                     // fs::rename(mp3_path,&target_dir_path).unwrap();
                      

                    }
                }
         
