#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

mod course;
mod module;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    Url: String,
    module_address:Address
}

/// Entry Definition
/// Entry Links


//// Helper function
pub fn add(url: String, moduleAddress: Address) -> ZomeApiResult<Address, ZomeApiError> {
    // Add content to module
    // Create Module_Content link
}

pub fn update(url: String, contentAddress: Address) -> ZomeApiResult<(), ZomeApiError> {
    // update content
}

pub fn delete(contentAddress: Address) -> ZomeApiResult<(), ZomeApiError> {
    // delete content
}

pub fn list(moduleAddress:Address) -> ZomeApiResult<vec!<Address>>{
    // list of content of module.
}
