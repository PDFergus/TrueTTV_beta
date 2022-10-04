use eframe::egui;
use native_dialog::{FileDialog,MessageDialog,MessageType};

fn main() {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
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
            ui.add_space(10.0);
            let mut bot_start =ui.add_sized([200.0, 100.0], egui::Button::new("Start Bots"));
            if bot_start.clicked(){
                let mut thingy = 0;
            }
            ui.horizontal(|ui|{
                ui.spacing_mut().item_spacing.x = 1.0;
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label("Configure Settings");
                let mut my_string = " ";
               
                

             }
            );
            ui.horizontal(|ui|{
                //let mut sound_path =" ";
                
                let sound_path =ui.add_sized([200.0, 20.0], egui::Button::new("Sounds")); //ui.text_edit_singleline(&mut self.sfx);
                if sound_path.clicked(){
                    let path = FileDialog::new()
                        .set_location("~/Desktop")
                        .add_filter("MP3 Audio", &["mp3"])
                        .show_open_single_file()
                        .unwrap();
                    let path = match path{
                        Some(path)=> path,
                        None => return,
                    };
                    let yes = MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Select File To Make new Command")
                        .set_text(&format!("{:#?}", path))
                        .show_confirm()
                        .unwrap();
                    if yes{
                        //todo 
                        let mut path_confirm = true;
                    }
                }
            });
            ui.horizontal(|ui|{
                ui.label("Command name:");
                let command_name = ui.add_sized([100.0,20.0], egui::TextEdit::singleline(&mut self.command_name));
                if command_name.changed(){
                    let some_command = true;
                }
            });
            ui.horizontal(|ui|{
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
                    let do_more_things = true;
                }
            });

            ui.horizontal(|ui|{
                ui.label("Enable Bit_bot");
                let bit_state = ui.checkbox(&mut self.bit_bot, "Checked");
                if bit_state.changed(){
                    //todo add to cfg file as all off as a base
                    let some_more_shis = true;
                }
            });
            ui.horizontal(|ui|{
                ui.label("Enable Sub Bot?");
                let sub_state = ui.checkbox(&mut self.sub_bot, "Checked");
                if sub_state.changed(){
                    //todo add logic to change config file
                    let some_more_thiongs = true;

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