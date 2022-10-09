use futures::TryFutureExt;
//use winapi::um::winuser::BlockInput;
use win32api_rs_sys::BlockInput;
use winconsole::errors::ArgumentError;
use std::path::Path;
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
use serde;
use serde_json::{self, Value};




//mod recorder;
#[tokio::main]

async fn parse_user() -> Value{
    let file = File::open("/config/default.json").expect("file should open read only");

    let json:serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    let user_name =  json.get("user_name").expect("File should have user_name key");
    return user_name.to_owned();
}
pub async fn twitch_irc() {
    

    
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message{
                ServerMessage::Privmsg(msg) =>{
                    
                    for badge in msg.badges{
                        //todo mut msg_text = msg.message_text.clone(); -> fn playsound -> create a sink per word in sentance.
                        let mut msg_text_clone = msg.message_text.clone().to_owned();
                         tokio::spawn(async move{
                                   play_sound(&msg_text_clone).await;
                               });
                               sleep(Duration::from_millis(10)).await;
                        for word in msg.message_text.split(" "){
                           
                            let mut word = word.to_owned();
                            let clone_word = word.clone();
                            let mut borrowed_word = word;
                            let mut borrowed_word_copy = clone_word;
                            

                            if badge.name == String::from("subscriber")||badge.name == String::from("founder"){
                               // tokio::spawn(async move{
                                 //  play_sound(&borrowed_word_copy).await;
                               // }).unwrap_or_else(|_| ());                 -------------------this is a fix
                                
                                if borrowed_word == "!minimize"{
                                    
                                    tokio::spawn(async move{
                                        return desktop_view().await;
                                    });
                                    sleep(Duration::from_millis(0)).await;      
                                }

                                else if borrowed_word == "!stay"{

                                    tokio::spawn(async move{
                                        key_send_looper().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                    
                                }
                                
                                //else if borrowed_word == "!yell"{

                                  //  tokio::spawn(async move{
                                    //    play_yell().await
                                   // });
                                   // sleep(Duration::from_millis(10)).await;                                   
                               // }


                                //else if word == "!nope"{

                                 // tokio::spawn(async move{
                                     //  unsafe{
                                       //   block_input().await
                                  //  }
                                 // });
                                  //  sleep(Duration::from_millis(10)).await;
                               // }

                                else if borrowed_word == "!slow"{

                                    tokio::spawn(async move{
                                        shift_walk().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                }

                                else if borrowed_word == "!dropit"{

                                    tokio::spawn(async move{
                                        drop_weapons().await
                                    });
                                    sleep(time::Duration::from_millis(10)).await;
                                }

                                else if borrowed_word == "!jump"{
                                    tokio::spawn(async move {
                                        
                                        return jump().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                }

                                else if borrowed_word == "!ducks"{

                                    tokio::spawn(async move{
                                        play_duck().await
                                    });
                                    sleep(Duration::from_millis(10)).await;                                   
                                }
                                
                           
                                
                            } 
                            
                        }
                        
                    }
                   
                },
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                },
                _ => {}
            }
        }
    });
    
    //let user_name = parse_user().to_string();
    client.join("whoopsitspete".to_owned()).unwrap();

    // keep the tokio executor alive.
    join_handle.await.unwrap();
}
async fn play_yell(){

    std::env::set_current_dir("/twitch_irc").expect("Can Not Find Directory Folder");
    sleep(time::Duration::from_millis(10)).await;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("sounds/!yell.mp3").unwrap());
    let source= Decoder::new(file).unwrap();
    //stream_handle.play_raw(source.convert_samples());
    sink.append(source);
    sink.sleep_until_end();
}
async fn play_duck(){

    std::env::set_current_dir("/twitch_irc").expect("Can Not Find Directory Folder");
    sleep(time::Duration::from_millis(10)).await;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("sounds/!ducks.mp3").unwrap());
    let source= Decoder::new(file).unwrap();
    //stream_handle.play_raw(source.convert_samples());
    sink.append(source);
    sink.sleep_until_end();
}
struct Play_Sounds{
    file_full_name:str,
}
async fn play_sound(msg_text_clone: &str){
    for word in msg_text_clone.split(" "){
        std::env::set_current_dir("/twitch_irc/sounds").expect("Can Not Find Directory Folder");
        sleep(time::Duration::from_millis(10)).await;
        let mut word_copy = word.clone();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).expect("The system cannot find the file specified.");
        let mut file_full_string = word_copy.to_owned() + ".mp3";
        let mut file_full_name = file_full_string.as_str();
        
        //let file_string_name_path:Path =Path::new(&file_full_name);
        let file = BufReader::new(File::open(file_full_name).unwrap());
        let source = Decoder::new(file).unwrap();
        //stream_handle.play_raw(source.convert_samples());
        sink.append(source);
        sink.sleep_until_end();
        
        

    }
    //std::env::set_current_dir("/twitch_irc").expect("Can Not Find Directory Folder");
    //sleep(time::Duration::from_millis(10)).await;
    //let mut file_dir = "sounds/".to_owned();
    //let mut file_name = borrowed_word_copy.to_owned();
    //let mut file_type = ".mp3";
    //file_name.push_str(file_type);
    //sleep(time::Duration::from_millis(10)).await;
    //let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //let sink = Sink::try_new(&stream_handle).unwrap();
    //let file = BufReader::new(File::open(file_name).expect("Could Not Find File"));
    //let source= Decoder::new(file).expect("No Matching Audio File in directory");
    //stream_handle.play_raw(source.convert_samples());
    //sink.append(source);
    //sink.sleep_until_end();
    
}

async fn shift_walk(){

    sleep(time::Duration::from_millis(10)).await;
    let mut start:u32 = 0;
    let end:u32 = 1;
    while start < end{
        winput::press(Vk::LeftShift);
        sleep(time::Duration::from_secs(5)).await;
        start += 1; 
    }
    winput::release(Vk::LeftShift);

  //  while start < end{
    //    winput::press(Vk::LeftShift);
      //  sleep(time::Duration::from_millis(30)).await;
       // start += 1;
   // }
   // winput::release(Vk::LeftShift);
}
async fn crouch_walk(){
    sleep(time::Duration::from_millis(10)).await;
    let mut start:i32 = 0;
    let end:i32 = 1;
    while start < end {
        winput::press(Vk::LeftControl);
        sleep(time::Duration::from_secs(2));
        start +=1;
    }
    winput::release(Vk::LeftControl);
}
async fn drop_weapons(){
    
    sleep(time::Duration::from_millis(10)).await;
    let mut start:u32 = 0;
    let end:u32 = 333;
    while start < end{
        winput::press(Vk::G);
        sleep(time::Duration::from_millis(30)).await;
        start += 1;
    }
    winput::release(Vk::G);    
}

async fn jump(){

    sleep(time::Duration::from_millis(30)).await;
    winput::press(Vk::Space);
    sleep(time::Duration::from_millis(10)).await;
    winput::release(Vk::Space);
}

async fn desktop_view(){
    
    sleep(time::Duration::from_millis(10)).await;
    winput::press(Vk::LeftWin);
    winput::send(Vk::D);
    winput::release(Vk::LeftWin);
}

async fn key_send_looper(){

    sleep(time::Duration::from_millis(10)).await;
    let mut start:u32 = 0;
    let end:u32 = 66;
    while start < end{
        winput::press(Vk::A);
        winput::press(Vk::W);
        winput::press(Vk::S);
        winput::press(Vk::D);
        sleep(time::Duration::from_millis(30)).await;
        start +=1;
    }
    winput::release(Vk::A);
    winput::release(Vk::W);
    winput::release(Vk::S);
    winput::release(Vk::D);
}

async unsafe fn block_input(){
    
    BlockInput(1);
    sleep(time::Duration::from_secs(1)).await;
    BlockInput(0);
 
}

//async fn sound_commands(word: &str){
  //  let word = word;
   // println!("(> . . )>{}", word);
//}

