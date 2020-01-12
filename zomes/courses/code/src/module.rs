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

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Module {
    Title: String
}

// Entry Definition
// Entry Link


///// Helper Function

pub fn add(title: string, courseAddress: Address) -> ZomeApiResult<Address, Error> {
    // Create Module entry
    // Create Course_Module link
    /// Retun Address 
}

pub fn update(title:String, moduleAddress:Address) -> ZomeApiResult<(),ZomeApiError>){
    // update module with new title
}

pub fn delete(moduleAddress::Address) -> ZomeApiResult<(),ZomeApiError>{
    // delete module
}

pub fn list(courseAddress:Address) -> ZomeApiResult<vec!<Address>>{

}