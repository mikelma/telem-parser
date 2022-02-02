use cobs;
use crate::*;

#[derive(Debug)]
pub struct TelemetryPacket(Vec<u8>);

#[derive(Debug, Clone)]
pub enum TelemField {
    Int32(i32),
    UInt32(u32),
    Float32(f32),
    Base40Str(String),
}

#[derive(Debug, Clone, Copy)]
pub enum TelemFieldType {
    Int32,
    UInt32,
    Float32,
    Base40Str,
}

impl TelemetryPacket {
    pub fn from(mut bytes: Vec<u8>) -> Result<Self, TelemError> {
        if bytes.len() < TELEMETRY_MIN_BYTES {
            return Err(TelemError::MissingBytes(bytes.len()));
        }

        if let Err(_) = cobs::decode_in_place(&mut bytes) {
            return Err(TelemError::CobsError);
        }

        bytes.pop();

        let pkg = TelemetryPacket(bytes);

        pkg.check_package()?;

        Ok(pkg)
    }

    fn check_package(&self) -> Result<(), TelemError> {
        /*
        if !self.check_beginning_of_package() {
            return Err(TelemError::MissingBOP);
        }

        if !self.check_end_of_package() {
            return Err(TelemError::MissingEOP);
        }
        */

        let n_fields = match self.read_field(TELEMETRY_FIELD_COUNT, TelemFieldType::UInt32)? {
            TelemField::UInt32(v) => v,
            _ => unreachable!(),
        };

        if (n_fields as usize + 1)*TELEMETRY_BYTES_PER_FIELD != self.0.len() {
            return Err(TelemError::LengthError);
        }

        let end = self.0.len() - TELEMETRY_BYTES_PER_FIELD as usize;
        let computed_crc = crc32::crc32(&self.0[0..end]);

        let mut crc_bytes = [0u8; 4];
        for (i, byte) in self.0[end..end+TELEMETRY_BYTES_PER_FIELD].iter().enumerate() {
            crc_bytes[i] = *byte;
        }

        let real_crc = u32::from_be_bytes(crc_bytes);

        if real_crc != computed_crc {
            return Err(TelemError::CrcError);
        }

        Ok(())
    }

    pub fn new(number_fields: usize) -> Result<TelemetryPacket, TelemError>  {
        if number_fields > TELEMETRY_MAX_FIELDS {
            return Err(TelemError::InvalidNumberOfFields(number_fields));
        }

        let total_len = number_fields + 1;

        let mut packet = TelemetryPacket(vec![0u8; total_len]);

        // write base data
        // packet.write_field(TelemField::UInt32(TELEMETRY_FIELD_START as u32), 0)?;
        // packet.write_field(TelemField::UInt32(TELEMETRY_END_MARKER as u32), total_len-1)?;

        let crc = crc32::crc32(&packet.0[..(total_len-1)*TELEMETRY_BYTES_PER_FIELD]);
        packet.write_field(TelemField::UInt32(crc), total_len-1)?;

        Ok(packet)
    }

    pub fn write_field(&mut self, value: TelemField, field_id: usize) -> Result<(), TelemError> {
        if field_id > self.0.len() {
            return Err(TelemError::InvalidFieldId(field_id, self.0.len()));
        }

        let start = field_id*TELEMETRY_BYTES_PER_FIELD;
        let end = start + TELEMETRY_BYTES_PER_FIELD;
        let bytes = match value {
            TelemField::Int32(v) => v.to_be_bytes(),
            TelemField::UInt32(v) => v.to_be_bytes(),
            TelemField::Float32(v) => v.to_be_bytes(),
            TelemField::Base40Str(v) => {
                todo!(); // TODO
            },
        };

        self.0[start..end].copy_from_slice(&bytes);

        Ok(())
    }

    pub fn read_field(&self, field_id: usize, field_ty: TelemFieldType) -> Result<TelemField, TelemError> {
        if field_id > self.0.len() {
            return Err(TelemError::InvalidFieldId(field_id, self.0.len()));
        }

        // read bytes corresponding to the selected `field_id`
        let mut data = [0u8; 4];
        for i in 0..TELEMETRY_BYTES_PER_FIELD {
            data[i] = self.0[field_id*TELEMETRY_BYTES_PER_FIELD + i];
        }

        // type conversion
        match field_ty {
            TelemFieldType::UInt32 => Ok(TelemField::UInt32(u32::from_be_bytes(data))),
            TelemFieldType::Int32 => Ok(TelemField::Int32(i32::from_be_bytes(data))),
            TelemFieldType::Float32 => Ok(TelemField::Float32(f32::from_be_bytes(data))),
            TelemFieldType::Base40Str => match base40::base40_decode(u32::from_be_bytes(data)) {
                Some(v) => Ok(TelemField::Base40Str(v)),
                None => Err(TelemError::Base40Decode),
            },
        }
    }

    /*
    fn check_beginning_of_package(&self) -> bool {
        let mut bytes = self.0.clone();
        println!("{:x?}", bytes);
        let start_marker = TELEMETRY_START_MARKER.to_be_bytes();

        while bytes.len() >= 4 {
            if start_marker == &bytes[0..4] {
                return true;
            }

            bytes.remove(0);
        }

        false
    }

    fn check_end_of_package(&self) -> bool {
        if self.0.len() < 4 { return false; }

        let end_marker = TELEMETRY_END_MARKER.to_be_bytes(); 

        for i in 0..self.0.len()-3 {
            if self.0[i..i+4] == end_marker {
                return true;
            }
        }

        false
    }
    */
}


