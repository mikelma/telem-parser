pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF;

    data.iter().for_each(|c| {
        let mut ch = *c;
        for _ in 0..8 {
            let b = (ch  as u32 ^ crc) & 1;
            crc >>= 1;

            if b != 0 {
                crc = crc ^ 0xEDB88320;
            }

            ch >>= 1;
        }
    });

    !crc
}
