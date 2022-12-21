use std::collections::VecDeque;

struct WT901 {
    pub ang: Option<(f32, f32, f32)>,
    pub mag: Option<(u32, u32, u32)>,
}

impl WT901 {
    pub fn new() -> Self {
        Self {
            ang: None,
            mag: None,
        }
    }
    pub fn cope_serial_data(&mut self, serial_buf: Vec<u8>) {
        let mut data = VecDeque::from(serial_buf);

        loop {
            if data.len() >= 11 {
                if data[0] != 0x55 {
                    //TODO: 0x55を先頭にずらす。
                    data.pop_front();

                    continue;
                }

                match data[1] {
                    0x54 => {
                        //println!("mag_X: {:?}, mag_Y: {:?}, mag_Z: {:?}",);

                        self.mag = Some((
                            as_u32_le(&[data[2], data[3]]),
                            as_u32_le(&[data[4], data[5]]),
                            as_u32_le(&[data[6], data[7]]),
                        ));
                    }
                    0x53 => {
                        self.ang = Some((
                            as_u32_le(&[data[2], data[3]]) as f32 / 32768.0 * 180.0,
                            as_u32_le(&[data[4], data[5]]) as f32 / 32768.0 * 180.0,
                            as_u32_le(&[data[6], data[7]]) as f32 / 32768.0 * 180.0,
                        ));
                        //println!("ang: {}, {}, {}",);
                    }
                    _ => {}
                }
            }
            break;
        }
    }
}


fn as_u32_le(array: &[u8; 2]) -> u32 {
    ((array[0] as u32) << 0) + ((array[1] as u32) << 8)
}
