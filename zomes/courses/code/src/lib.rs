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

#[zome]
mod Course {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    /// Course Entry Definition
    /// Module Entry Definition
    /// Content Entry Definition 
    

    ////////// Course Functions
    // add
    // delete
    // update 
    // list

    /////// Module Functions
    /// add 
    /// delete
    /// update
    /// list(courseAddress)
    

    ///// Content Functions
    /// add
    /// delete
    /// update
    /// list(Module Functions)
}
