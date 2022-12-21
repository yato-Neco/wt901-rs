use std::collections::VecDeque;


pub fn cope_serial_data(serial_buf: Vec<u8>){
    let mut data = VecDeque::from(serial_buf);
    loop{
        if data.len() >= 11 {
            if data[0] != 0x55 {
                //TODO: 0x55を先頭にずらす。
                data.pop_front();
                
                continue;
            }
           
            

            match data[1]  {
                0x54 => {
                    println!("mag_X: {:?}, mag_Y: {:?}, mag_Z: {:?}",as_u32_le(&[data[2],data[3]]),as_u32_le(&[data[4],data[5]]),as_u32_le(&[data[6],data[7]]));
                }
                0x53 => {
                    println!("ang: {}, {}, {}",as_u32_le(&[data[2],data[3]]) as f32 /32768.0*180.0 ,as_u32_le(&[data[4],data[5]]) as f32 /32768.0*180.0,as_u32_le(&[data[6],data[7]]) as f32/32768.0*180.0);
                }
                _ => {}
            }

            

        }
        break;
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
     
    }
}


fn as_u32_le(array: &[u8; 2]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8)
}