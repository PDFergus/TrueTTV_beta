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



//mod recorder;
#[tokio::main]
pub async fn twitch_sub() {
    
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message{
                ServerMessage::Privmsg(msg) =>{
                    let mut msg_text_clone = msg.message_text.clone().to_owned();
                         //tokio::spawn(async move{
                           //        play_sounds(&msg_text_clone).await;
                             //  });
                              // sleep(Duration::from_millis(10)).await;
                    
                    for badge in msg.badges{
                        for word in msg.message_text.split(" "){
                           // tokio::spawn(async move{
                             //   let new_word = String::from( word);
                               // play_sound(new_word).await;
                            //});

                            if badge.name == String::from("subscriber")||badge.name == String::from("founder"){
                                
                                if word == "!minimize"{

                                    tokio::spawn(async move{
                                        desktop_view().await;
                                    });
                                    sleep(Duration::from_millis(10)).await;      
                                }

                                else if word == "!stay"{

                                    tokio::spawn(async move{
                                        key_send_looper().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                    
                                }
                                
                                else if word == "!yell"{

                                    tokio::spawn(async move{
                                        play_yell().await
                                    });
                                    sleep(Duration::from_millis(10)).await;                                   
                                }

                               // else if word == "!nope"{

                                 //   tokio::spawn(async move{
                                   //     unsafe{
                                    //        block_input().await
                                     //   }
                                   // });
                                   // sleep(Duration::from_millis(10)).await;
                               // }

                                else if word == "!slow"{

                                    tokio::spawn(async move{
                                        shift_walk().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                }

                                else if word == "!dropit"{

                                    tokio::spawn(async move{
                                        drop_weapons().await
                                    });
                                    sleep(time::Duration::from_millis(10)).await;
                                }

                                else if word == "!jump"{
                                    tokio::spawn(async move{
                                        jump().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                }

                                else if word == "!ducks"{

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
    let user_name = parse_user();
    
    client.join(user_name.to_string()).unwrap();

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
        std::env::set_current_dir("eframe_test/sounds").expect("Can Not Find Directory Folder");
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
  
}

async fn play_yell(){

    std::env::set_current_dir("/eframe_test").expect("Can Not Find Directory Folder");
    sleep(time::Duration::from_millis(10)).await;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("sounds/angryscav.mp3").unwrap());
    let source= Decoder::new(file).unwrap();
    //stream_handle.play_raw(source.convert_samples());
    sink.append(source);
    sink.sleep_until_end();
}
async fn play_duck(){

    std::env::set_current_dir("/eframe_test").expect("Can Not Find Directory Folder");
    sleep(time::Duration::from_millis(10)).await;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("sounds/3ducks.mp3").unwrap());
    let source= Decoder::new(file).unwrap();
    //stream_handle.play_raw(source.convert_samples());
    sink.append(source);
    sink.sleep_until_end();
}



async fn shift_walk(){

    sleep(time::Duration::from_millis(10)).await;
    let mut start:u32 = 0;
    let end:u32 = 333;
    winput::press(Vk::LeftShift);
    sleep(time::Duration::from_secs(5)).await;
    winput::release(Vk::LeftShift);

  //  while start < end{
    //    winput::press(Vk::LeftShift);
      //  sleep(time::Duration::from_millis(30)).await;
       // start += 1;
   // }
   // winput::release(Vk::LeftShift);
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
        sleep(time::Duration::from_millis(15)).await;
        start +=1;
    }
    winput::release(Vk::A);
    winput::release(Vk::W);
    winput::release(Vk::S);
    winput::release(Vk::D);
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

