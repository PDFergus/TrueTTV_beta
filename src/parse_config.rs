use std::fs::File;

use serde;
use serde_json::{self, Value};

pub fn parse_user() -> String{
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

pub fn parse_bit_bot() -> bool{
    let file = File::open("default.json").expect("file should open read only");

    let json:serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    let bit_stat =  json.get("bit_bot_enabled").expect("File should have user_name key");
    let bit_stat = bit_stat.to_string();
    
    let bit_stat = bit_stat.replace(r#"'"#,"");
    let bit_stat = bit_stat.replace(r#"/"#, "");
    let bit_stat = bit_stat.replace(r#"""#, "");
    let bit_stat = bit_stat.trim().to_owned();
    if bit_stat != "true"{
        return false;
    }
    else{
        return true;
    }
}

pub fn parse_sub_bot() -> bool{
    let file = File::open("default.json").expect("file should open read only");

    let json:serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    let mut sub_stat =  json.get("sub_bot_enabled").expect("File should have user_name key");
    
    let mut sub_stat = sub_stat.to_string();
    let mut sub_stat = sub_stat.replace(r#"'"#,"");
    let mut sub_stat = sub_stat.replace(r#"/"#, "");
    let mut sub_stat = sub_stat.replace(r#"""#, "");
    let sub_stat = sub_stat.trim().to_owned();
    if sub_stat != "true"{
        return false;
    }
    
    else{
        return true;
    }
}

pub fn parse_sound_bot() -> bool{
    let file = File::open("default.json").expect("file should open read only");

    let json:serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    let mut sound_stat =  json.get("sound_bot_enabled").expect("File should have user_name key");
    let mut sound_stat = sound_stat.to_string();
    let mut sound_stat = sound_stat.replace(r#"'"#,"");
    let mut sound_stat = sound_stat.replace(r#"/"#, "");
    let mut sound_stat = sound_stat.replace(r#"""#, "");
    let mut sound_stat = sound_stat.trim().to_owned();
    
   if sound_stat != "true"{
        return false;
   }
   else{
        return true;
   }

    
}