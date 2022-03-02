use std::collections::HashMap;
use crate::TelemError;

use super::TelemFieldType;

use serde_json;
use serde_derive::{Deserialize, Serialize};

use std::fs;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub pkgs: Vec<PacketType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PacketType {
    pub name: String,
    pub id: usize,
    pub number_fields: usize,
    pub fields: Vec<Field>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub index: usize,
    pub ty: TelemFieldType,
    
}

impl Config {
    pub fn read(path: &str) -> Result<Self, TelemError> {
        match fs::read_to_string(path) {
            Ok(val) => match serde_json::from_str::<Config>(&val) {
                Ok(cfg) => Ok(cfg),
                Err(e) => Err(TelemError::CfgParse(e.to_string())),
            },
            Err(e) => Err(TelemError::CfgRead(e.to_string())),
        }
    }

    pub fn get_type<'a>(&'a self, id: usize) -> Result<&'a PacketType, TelemError> {
        match self.pkgs.iter().find(|pkg| pkg.id == id) {
            Some(t) => Ok(t),
            None => Err(TelemError::PkgTypeNotFound(id)),
        }
    }

    /*
    pub fn test() {
        let pkgs = vec![
            PacketType {
                name: "telemetry".into(),
                id: 0x77777777,
                fields: vec![
                    Field {
                        name: "gps".into(),
                        index: 0,
                        ty: TelemFieldType::Int32
                    },
                    Field {
                        name: "barometer".into(),
                        index: 1,
                        ty: TelemFieldType::Float32,
                    }
                ],
            },
        ];

        let cfg = Config { pkgs };
        println!("{}", serde_json::to_string_pretty(&cfg).unwrap());
    }
    */
}

impl PacketType {
    pub fn get_name(&self) -> String {
        self.name.clone() 
    }

    pub fn get_field<'a>(&'a self, field_name: &str) -> Result<&'a Field, TelemError> {
        match self.fields.iter().find(|field| field.name == field_name) {
            Some(t) => Ok(t),
            None => Err(TelemError::FieldNotFound(field_name.to_string())),
        }
    }
}
