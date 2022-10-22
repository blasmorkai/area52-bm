use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use cw_storage_plus::Item;
use crate::species::{Sapient, SapienceScale};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}

// static   creates a constant, besides, the data pointed to by the reference lives for the entire lifetime of the running program
// b        tells the compiler that the string should be treated as a byte sequence. Converts &str to bytes
static CONFIG_KEY: &[u8] = b"config";

pub fn config(storage:&mut dyn Storage) -> Singleton<State> {
    Singleton::new(storage,CONFIG_KEY)  // new() takes a &[u8] key parameter, basically a sequence of bytes.
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    ReadonlySingleton::new(storage,CONFIG_KEY)
}