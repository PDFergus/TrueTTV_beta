//use winapi::um::winuser::BlockInput;

use serde_json::Value;
use::serde;

use std::{thread, time};
use std::env;
use std::fs::File;
use std::io::BufReader;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::{IRCMessage, ServerMessage};
use std::process::Command;
use winput::{Vk, Button};
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use tokio::time::{sleep, Duration};

use crate::parse_config;



//mod recorder;
#[tokio::main]
pub async fn twitch_sound() {
    
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message{
                ServerMessage::Privmsg(msg) =>{
                    let mut msg_text_clone = msg.message_text.clone().to_owned();
                         tokio::spawn(async move{
                                   play_sounds(&msg_text_clone).await;
                               });
                               sleep(Duration::from_millis(10)).await;

                   
                },
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                },
                _ => {}
            }
        }
    });
    let user_name = parse_config::parse_user();//parse_user();
    
    client.join(user_name).unwrap();

    // keep the tokio executor alive.
    join_handle.await.unwrap();
}

fn parse_user() -> String{
    let file = File::open("default.json").expect("file should open read only");

    let json:serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    let user_name =  json.get("user_name").expect("File should have user_name key");
    let user_name = user_name.to_string();
    let user_name = user_name.replace(r#"'"#,"");
    let user_name =user_name.replace(r#"/"#, "");
    let user_name = user_name.replace(r#"""#, "");
    let user_name = user_name.trim().to_owned();

    return user_name;
}

async fn play_sounds(msg_text_clone: &str){
    for word in msg_text_clone.split(" "){
        //std::env::set_current_dir("sounds").expect("Can Not Find Directory Folder");
        sleep(time::Duration::from_millis(10)).await;
        let mut word_copy = word.clone();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).expect("The system cannot find the file specified.");
        let mut file_full_string = word_copy.to_owned() + ".mp3";
        let mut file_full_name = file_full_string.as_str();
        let mut file_path = ("sounds/").to_owned() + file_full_name;
        
        //let file_string_name_path:Path =Path::new(&file_full_name);
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        //stream_handle.play_raw(source.convert_samples());
        sink.append(source);
        sink.sleep_until_end();
        
        

    }
  
}


//async unsafe fn block_input(){
    
   // BlockInput(1);
   // sleep(time::Duration::from_secs(1)).await;
  //  BlockInput(0);
 
//}

//async fn sound_commands(word: &str){
  //  let word = word;
   // println!("(> . . )>{}", word);
//}

