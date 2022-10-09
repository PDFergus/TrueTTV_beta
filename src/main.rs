mod file_dialog;
mod twitch_irc_sub_bot;
mod parse_config;
use eframe::egui;
use std::env;
use std::env::set_current_dir;
use std::fs;
use std::fs::File;
use std::io::prelude;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use native_dialog::{FileDialog,MessageDialog,MessageType};

use std::io::Read;
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
fn main() {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Debug, Deserialize, Serialize)]
struct TempDefaults{

    user_name: Vec<User_Name>,
    bit_bot_enabled: String,
    sub_bot_enabled:String,
    sound_bot_enabled:String,

}
#[derive(Debug, Deserialize, Serialize)]
struct User_Name{
    user_name: String,
}

#[derive(Default)]
struct MyEguiApp{
    ttv:String,
    value: f32,
    sfx:String,
    message_response:String,
    user:String,
    bit_bot:bool,
    bit_status:String,
    sub_bot:bool,
    sub_status:String,
    sound_bot:bool,
    sound_status:String,
    font_height:f32,
    command_name:String,
    info_string:String,
    
    target_dir:String,
    command_name_renamer:String,
    new_filename:String,
    final_dir:String,
    command_file_name:String,
    command_file_name_str:String,
    path:PathBuf,
    
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
       let mut ctx = egui::Context::default();
       ctx.set_visuals(egui::Visuals::default());
      
       Self::default()
       
    }
}

impl eframe::App for MyEguiApp{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
            ui.menu_button("file", |ui|{
                if ui.button("Quit").clicked(){
                    frame.close();
                }
                //if ui.button("Add custom Script(HARD BETA)").clicked(){
                    //todo
                //}
                
            });
        });
        
        egui::SidePanel::left("side_panel").show(ctx, |ui|{
           // if ui.add_sized([400.0, 400.0], button.clicked()){
            //.button("start bots").clicked() {
             //   let mut thinggy = 0;
            ui.heading("Quick Settings");
            ui.add_space(10.0);
            let mut bot_start =ui.add_sized([200.0, 100.0], egui::Button::new("Start Bots"));
            if bot_start.clicked(){
                thread::spawn(|| {
                    twitch_irc_sub_bot::twitch_irc();  
                }); 
            }
          
            
            ui.label("Add Sound Commands");
            
            
            ui.vertical(|ui|{
                ui.separator();
                ui.label("Command name:");
                let command_name = ui.add_sized([100.0,20.0], egui::TextEdit::singleline(&mut self.command_name));
                if command_name.changed(){
                    let some_command = true;
                    let temp_command_name = &mut self.command_name.to_string();
                    let mut new_filename = temp_command_name.push_str(".mp3");
                }
            });
            ui.vertical(|ui|{
                //let mut sound_path =" ";
                
                //let sound_settings_sep = ui.add(egui::Separator::horizontal(Self));
                let sound_path =ui.add_sized([200.0, 20.0], egui::Button::new("Select .MP3")); //ui.text_edit_singleline(&mut self.sfx);
                if sound_path.clicked(){
                    let command_name_clone = self.command_name.clone();
                    thread::spawn( move ||{
                        file_dialog::pub_dialog(&command_name_clone);

                    });
                  
                }
            });
           
            ui.add_space(10.00);
            ui.label("Twitch Bot Settings");
            ui.separator();
            ui.vertical(|ui|{
                
                ui.label("TTV name:");
                let channel_name = ui.add_sized([100.00, 20.0], egui::TextEdit::singleline(&mut self.ttv));//text_edit_singleline(&mut self.ttv);
                if channel_name.changed(){
                    //do_something(path)
                    let some_thing_happens_again  = true;
                }
            });

            ui.horizontal(|ui|{
                ui.label("Enable Sound Bot: ");
                let sound_state = ui.checkbox(&mut self.sound_bot, "Checked");
                if sound_state.changed(){
                    let mut sound_bot = true;
                }
            });

            ui.horizontal(|ui|{
                ui.label("Enable Bit_bot");
                let bit_state = ui.checkbox(&mut self.bit_bot, "Checked");
                if bit_state.changed(){
                    //todo add to cfg file as all off as a base
                    let mut bit_bot = true;
                }
            });
            ui.horizontal(|ui|{
                ui.label("Enable Sub Bot?");
                let sub_state = ui.checkbox(&mut self.sub_bot, "Checked");
                if sub_state.changed(){
                    //todo add logic to change config file
                    
                    let mut sub_bot = true;
                    

                }
            });
            ui.horizontal(|ui|{
                let edit_configs = ui.add_sized([200.0,40.0], egui::Button::new("Edit Configs"));
                if edit_configs.clicked(){
                    let mut current_user = self.ttv.clone();
                    let mut bit_stat_config = self.bit_bot.clone();
                    let mut sub_stat_config = self.sub_bot.clone();
                    let mut sound_state_config =  self.sound_bot.clone();
                    thread::spawn(move||{
                        //current default settings 
                        let mut default_user = parse_config::parse_user();
                        let mut default_bb = parse_config::parse_bit_bot();
                        let mut default_sb = parse_config::parse_sound_bot();
                        let mut default_sub = parse_config::parse_sub_bot();

                        
                        //changed settings
                        let current_user_clone = current_user.to_owned();
                        if &default_user == &current_user.to_string() && &default_bb == &bit_stat_config && &default_sub == &sub_stat_config.to_owned() && &default_sb == &sound_state_config.to_owned() {
                            return;
                        }
                        else{
                            //set_current_dir("eframe_test").unwrap();
                            let bit_stat = &bit_stat_config.to_string();
                            let sub_stat = &sub_stat_config.to_string();
                            let sound_stat = &sound_state_config.to_string();
                            let json = json!({
                                "user_name": &current_user,
                                "bit_bot_enabled": &bit_stat,
                                "sub_bot_enabled": &sub_stat,
                                "sound_bot_enabled": &sound_stat

                            });

                            std::fs::write(
                                "default_temp.json",
                                serde_json::to_string_pretty(&json).unwrap(),
                            ).unwrap();

                            fs::remove_file("default.json").expect("No file found");
                            fs::rename("default_temp.json","default.json").unwrap();
                            return;

                           // for user in defa
                           
                         //  let mut defaults = {
                           //     let file = File::open("default.json").expect("file should open read only");
                             //   let base_input:serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
                             //   let base_input = base_input.to_string();
                              // // let base_input = base_input.replace(r#"'"#,"");
                              // // let base_input = base_input.replace(r#"/"#, "");
                               // //let base_input = base_input.replace(r#"""#, "");
                               // //let base_input = base_input.trim().to_owned();
                                //serde_json::from_str::<TempDefaults>(&base_input).unwrap();
                          // };
                           //for index in 0..defaults.to_owned().User_Name.len(){

                           }
                           
                        
                        
                       // let mut bit_stat_config =  &mut self.bit_bot.to_string();
                       // let mut sub_stat_config = &mut self.sub_bot.to_string();
                       // let mut sound_stat_config = &mut self.sound_bot.to_string();

                        
                        


                    });
                    
                    
                }
            });
            
                

        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            
            let intro_info = ui.add_space(40.0);
            ui.label("  Hello, and welcome to True Twitch.TV. A streamer Focused Solution for your cloud based woes,

            keep in mind that this project is in early developement and as a result may have a few *bugs*
            Lets start with a tour. on the left is the basic configuration settings.
            using the Add Sounds button and the Command name field in conjunction will make a new sound command from any mp3 file! just like magic! 
            after the simple command adding there are configuration options available for the Bit Bot, Sub Bot, and Sound Bot Enabled. Bellow is the IRC Reader so you wont have to keep six windows open to know everything is running!
            Future updates hope to see this large and lovely area filled with a ffmpeg video codec for streaming your content to know what you are watching
            
            Big Love Gamer Energy, Pete");
            
            ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.info_string));
            
        });

    }
}