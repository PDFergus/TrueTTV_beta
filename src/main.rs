mod file_dialog;
mod sub_bot;
mod sound_bot;
mod bit_bot;

mod twitch_irc_sub_bot_complete;

use twitch_irc_sub_bot_complete::twitch_irc;
use bit_bot::twitch_bb;
use sub_bot::twitch_sub;
use sound_bot::twitch_sound;

mod parse_config;

use eframe::egui::{self, InnerResponse};
use eframe::epaint::Color32;
use parse_config::parse_sub_bot;
use std::env::set_current_dir;
use std::fs::{File, self};
use std::path::PathBuf;
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
fn main() {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("TrueTTV", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct StatusImageOn{
    status:Option<egui::TextureHandle>
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
    status_on:Option<egui::TextureHandle>,
    target_dir:String,
    command_name_renamer:String,
    new_filename:String,
    final_dir:String,
    command_file_name:String,
    command_file_name_str:String,
    path:PathBuf,
    is_on:bool,
    state:String,
    sound_on:String,
    
   

    
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
                    let mut is_on = false;
                    let json = json!({
                    "bot_status": &is_on,
                    });

                std::fs::write(
                    "on_state.json",
                    serde_json::to_string_pretty(&json).unwrap(),
                ).unwrap();

            
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
            //let mut state = "off";
            //let mut state = state.to_string();
            //self.state=state;
            ui.add_space(10.0);
            let status = parse_config::parse_on_status();
            
            let mut bot_start =ui.add_sized([200.0, 100.0], egui::Button::new("Start Bots"));
            if bot_start.clicked(){
               // let mut is_on = true;
                //let json = json!({
                //    "bot_status": &is_on,
                //});

              //  std::fs::write(
               //     "on_state.json",
               //     serde_json::to_string_pretty(&json).unwrap(),
              //  ).unwrap();

              let mut sub_on = parse_config::parse_sub_bot();
              let mut sound_on = parse_config::parse_sound_bot();
              let mut bit_on =parse_config:: parse_bit_bot();
           
              if sub_on == true && sound_on == true && bit_on == true{
               thread::spawn(|| {    
                   twitch_irc_sub_bot_complete::twitch_irc();   
                   });  
              }
              else if sub_on != true{
               thread::spawn(|| {    
                   sound_bot::twitch_sound();   
                   });
               thread::spawn(||{
                   bit_bot::twitch_bb();
               });
               }else if sound_on != true{
                   thread::spawn(||{
                       bit_bot::twitch_bb();
                   });
                   thread::spawn(||{
                       sub_bot::twitch_sub();
                   });
               }else if bit_on != true{
                   thread::spawn(||{
                       sound_bot::twitch_sound();
                   });
                   thread::spawn(||{
                    
                    sub_bot::twitch_sub();
                });
               }

                let mut cstate = "";
                let mut cstate = cstate.to_string();
                self.state = cstate;

                let mut state = "on";
                let mut state = state.to_string();
                self.state = state;
                
                
            }
            let command_name = ui.add_sized([50.0,20.0], egui::TextEdit::singleline(&mut self.state));
            let status = parse_config::parse_on_status();
            
                
                
                
            
            
            
          
            
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
            ui.separator();
            ui.vertical(|ui|{
                    ui.label("Enable Sound Bot: ");
                    ui.vertical(|ui|{
                        let command_name = ui.add_sized([50.0,20.0], egui::TextEdit::singleline(&mut self.sound_on));
                    });
                    let mut status = parse_config::parse_sound_bot();
                    if status == false{
                        &mut *self.sound_on;
                        let mut cstate = "off";
                        let mut cstate = cstate.to_string();
                        self.sound_on = cstate.clone();
                        
                    }else{
                        &mut *self.sound_on;
                        let mut sound_on_state = "on";
                        let mut sound_on_state = sound_on_state.to_string();
                        self.sound_on=sound_on_state.clone();

                    }

                    
                
                 
                
                let sound_state = ui.add_sized([200.0,20.0], egui::Button::new("enable"));//checkbox(&mut self.sound_bot, "Checked");
                
                
                
                if sound_state.clicked(){
                    &mut *self.sound_on;
                    let mut sound_state = "on";
                    let mut state = sound_state.to_string();
                    self.sound_on = state.clone();
                    self.sound_bot = true;

                    
                }

                let sound_state_off = ui.add_sized([200.0,20.0], egui::Button::new("disabled"));
                if sound_state_off.clicked(){
                   &mut *self.sound_on;
                    let mut sound_state = "off";
                    let mut sound_state = sound_state.to_string();
                    self.sound_on = sound_state.clone();
                    self. sound_bot = false;
                   // self.sound_on = cstate.clone();
                        
                    self.sound_bot = false;
                }
                
                
            });
            
               
           
            
            ui.vertical(|ui|{
                ui.label("Enable Bit Bot");
                //checkbox(&mut self.bit_bot, "Checked");
                
                ui.horizontal(|ui|{
                   
                    let texture = self.status_on.get_or_insert_with(||{
                        let status = parse_config::parse_bit_bot();
                        if status == false{
                            ui.ctx().load_texture(
                                "Status Off", 
                                egui::ColorImage::new([200 ,10 ], Color32::from_rgb( 245,  89,  47)) ,
                                egui::TextureFilter::Linear)}
                        else{
                            ui.ctx().load_texture(
                            
                                "Status On", 
                                egui::ColorImage::new([200 ,10 ], Color32::from_rgb( 103,   245,   47)) ,
                                egui::TextureFilter::Linear)
                        }
                    });
                    let texture_clone = texture.clone();
                    ui.add(egui::Image::new(texture,texture_clone.size_vec2()));
                    
                });
                let bit_state =  ui.add_sized([200.0,20.0], egui::Button::new("enable"));
                if bit_state.clicked(){
                    //todo add to cfg file as all off as a base
                    self.bit_bot = true;
                }
                let bit_state_off = ui.add_sized([200.0,20.0], egui::Button::new("disable"));
                    if bit_state_off.clicked(){
                        self.bit_bot = false;
                    }
                
            });
            
            ui.vertical(|ui|{
                ui.label("Enable Sub Bot?");
                
                
                ui.horizontal(|ui|{
                   
                    let texture = self.status_on.get_or_insert_with(||{
                        let status = parse_config::parse_sub_bot();
                        if status == false{
                            ui.ctx().load_texture(
                                "Status Off", 
                                egui::ColorImage::new([200 ,10 ], Color32::from_rgb( 245,  89,  47)) ,
                                egui::TextureFilter::Linear)}
                        else{
                            ui.ctx().load_texture(
                            
                                "Status On", 
                                egui::ColorImage::new([200 ,10 ], Color32::from_rgb( 103,   245,   47)) ,
                                egui::TextureFilter::Linear)
                        }
                    });
                    let texture_clone = texture.clone();
                    ui.add(egui::Image::new(texture,texture_clone.size_vec2()));
                    
                });
                let sub_state = ui.add_sized([200.0,20.0], egui::Button::new("enable"));//checkbox(&mut self.sub_bot, "Checked");
                if sub_state.clicked(){
                    
                    
                   self.sub_bot = true;
                    

                }
                
                let sub_state_off = ui.add_sized([200.0,20.0], egui::Button::new("disable"));
                if sub_state_off.clicked(){
                     self.sub_bot = false;
                }
                
                
            });
            ui.horizontal(|ui|{
                let edit_configs = ui.add_sized([200.0,40.0], egui::Button::new("Edit Configs"));
                if edit_configs.clicked(){
                    
                    let mut temp_file = File::create("default_temp.json").unwrap();
                    let mut current_user = self.ttv.clone();
                    let mut bit_stat_config = self.bit_bot.clone();
                    let mut sub_stat_config = self.sub_bot.clone();
                    let mut sound_state_config =  self.sound_bot.clone();
                    let bot_state_config = self.is_on.clone();
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
                            let bot_state = &bot_state_config.to_string();
                            let json = json!({
                                "bot_status": &bot_state,
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
            ui.heading("Hello Gamers!");
            
            let intro_info = ui.add_space(40.0);
            ui.label("  Hello, and welcome to True Twitch.TV. A streamer Focused Solution for your cloud based woes,

                keep in mind that this project is in early developement and as a result may have a few *bugs*
                Lets start with a tour. on the left is the basic configuration settings.
                using the Add Sounds button and the Command name field in conjunction will make a new sound command from any mp3 file! just like magic! 
                after the simple command adding there are configuration options available for the Bit Bot, Sub Bot, and Sound Bot Enabled. B
                ellow is the IRC Reader so you wont have to keep six windows open to know everything is running!
                Future updates hope to see this large and lovely area filled with a ffmpeg video codec for streaming your content to know what you are watching
            
                Big Love Gamer Energy, Pete");
            
            ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.info_string));
            
        });

    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}