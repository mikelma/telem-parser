use cobs;
use crate::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug)]
pub struct TelemetryPacket {
    data: Vec<u8>,
    cfg: PacketType,
}

#[derive(Debug, Clone)]
pub enum TelemField {
    Int32(i32),
    UInt32(u32),
    Float32(f32),
    Base40Str(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TelemFieldType {
    Int32,
    UInt32,
    Float32,
    Base40Str,
}

impl TelemetryPacket {
    pub fn from(mut bytes: Vec<u8>, config: &Config) -> Result<Self, TelemError> {
        if bytes.len() < TELEMETRY_MIN_BYTES {
            return Err(TelemError::MissingBytes(bytes.len()));
        }

        if let Err(_) = cobs::decode_in_place(&mut bytes) {
            return Err(TelemError::CobsError);
        }

        bytes.pop();

        let type_id = match Self::read_field_raw(&bytes, TELEMETRY_FIELD_TYPE, TelemFieldType::UInt32)? {
            TelemField::UInt32(v) => v,
            _ => unreachable!(),
        };

        let cfg_ty = config.get_type(type_id as usize)?;

        let pkg = TelemetryPacket {
            data: bytes,
            cfg: (*cfg_ty).clone(),
        };

        pkg.check_package()?;

        Ok(pkg)
    }

    pub fn get_raw(&self) -> &[u8] {
        &self.data
    }

    fn check_package(&self) -> Result<(), TelemError> {
        let n_fields = match Self::read_field_raw(&self.data, TELEMETRY_FIELD_COUNT, TelemFieldType::UInt32)? {
            TelemField::UInt32(v) => v,
            _ => unreachable!(),
        };

        if (n_fields as usize + 1)*TELEMETRY_BYTES_PER_FIELD != self.data.len() {
            return Err(TelemError::LengthError);
        }

        let end = self.data.len() - TELEMETRY_BYTES_PER_FIELD as usize;
        let computed_crc = crc32::crc32(&self.data[0..end]);

        let mut crc_bytes = [0u8; 4];
        for (i, byte) in self.data[end..end+TELEMETRY_BYTES_PER_FIELD].iter().enumerate() {
            crc_bytes[i] = *byte;
        }

        let real_crc = u32::from_be_bytes(crc_bytes);

        if real_crc != computed_crc {
            return Err(TelemError::CrcError);
        }

        Ok(())
    }

    pub fn read_field(&self, field_name: &str) -> Result<TelemField, TelemError> {
        let field = self.cfg.get_field(field_name)?;
        self.read_field_by_id(field.index, field.ty)
    }

    pub fn read_field_by_id(&self, field_id: usize, field_ty: TelemFieldType) -> Result<TelemField, TelemError> {
        Self::read_field_raw(&self.data, field_id, field_ty)
    }

    fn read_field_raw(data: &[u8], field_id: usize, field_ty: TelemFieldType) -> Result<TelemField, TelemError> {
        if field_id > data.len() {
            return Err(TelemError::InvalidFieldId(field_id, data.len()));
        }

        // read bytes corresponding to the selected `field_id`
        let mut field_data = [0u8; 4];
        for i in 0..TELEMETRY_BYTES_PER_FIELD {
            field_data[i] = data[field_id*TELEMETRY_BYTES_PER_FIELD + i];
        }

        // type conversion
        match field_ty {
            TelemFieldType::UInt32 => Ok(TelemField::UInt32(u32::from_be_bytes(field_data))),
            TelemFieldType::Int32 => Ok(TelemField::Int32(i32::from_be_bytes(field_data))),
            TelemFieldType::Float32 => Ok(TelemField::Float32(f32::from_be_bytes(field_data))),
            TelemFieldType::Base40Str => match base40::base40_decode(u32::from_be_bytes(field_data)) {
                Some(v) => Ok(TelemField::Base40Str(v)),
                None => Err(TelemError::Base40Decode),
            },
        }
    }
}
