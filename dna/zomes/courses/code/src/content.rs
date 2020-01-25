/************************ Import Required Libraries */
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult, AGENT_ADDRESS};

use crate::course;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use std::convert::TryFrom;

/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    name: String,
    url: String,
    descritpion: String,
}

/*

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
*/
