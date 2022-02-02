pub fn base40_decode(code: u32) -> Option<String> {

    if code > 0xF423FFFF {
        return None;
    }
    
    let mut decoded = String::new();

    let mut s: u8;
    let mut mcode: u32 = code;

    while mcode > 0 { 
        s = (mcode % 40) as u8;
        if s == 0 { 
            decoded.push('-');
        } else if s < 11 {
            decoded.push((b'0' + s - 1) as char);
        } else if s < 14 {
            decoded.push('-');
        } else {
            decoded.push((b'A' + s - 14) as char);
        }
        mcode /= 40;
    }

    Some(decoded)
}
