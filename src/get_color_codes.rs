


use crate::parse_config::{parse_sound_bot,parse_bit_bot, parse_sub_bot};

//gets u8 colors for sound bot indicator


pub  fn get_color_r_sound() -> u8{
    let mut status = parse_sound_bot();
    if status == false{
        return 245
    }
    else{
        return 103
    }
    //do the true false check for each of the on off booleans
}
pub fn get_color_g_sound() -> u8{
   let mut status = parse_sound_bot();
   if status == false{
        return 103
   }
   else{
        return 245
   }
}
pub fn get_color_b_sound()->u8{
    return 47
}

//gets u8 colors for sub lights
pub fn get_color_r_sub()->u8{

    let mut status = parse_sub_bot();
    if status == false{
        return 245
    }else{
        return 103
    }

}
pub fn get_color_g_sub() -> u8{
    let mut status = parse_sub_bot();
    if status == false{
        return 103
    }else{
        return 245
    }

}

pub fn get_color_b_sub()->u8{
    return 47
}
//gets u8 colors for bit bot indicators
pub fn get_color_r_bits()->u8{
    let mut status = parse_bit_bot();
    if status == false{
        return 245
    }else{
        return 103
    }

}

pub fn get_color_g_bits()->u8{
    let mut status = parse_bit_bot();
    if status == false{
        return 103
    }else{
        return 245
    }

}

pub fn get_color_b_bits()->u8{
    return 47
}