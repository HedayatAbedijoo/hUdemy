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

  /**************************** Course Publish Function From Zome to OutSide */
  #[entry_def]
  fn anchor_entry_definition() -> ValidatingEntryType {
    course::anchor_entry_def()
  }

  #[entry_def]
  fn course_entry_definition() -> ValidatingEntryType {
    course::course_entry_def()
  }

  #[zome_fn("hc_public")]
  fn create_course(title: String) -> ZomeApiResult<Address> {
    course::create(&title)
  }

  #[zome_fn("hc_public")]
  fn update_course(title: String, courseAddress: Address) -> ZomeApiResult<Address> {
    course::update(&title, &courseAddress)
  }

  #[zome_fn("hc_public")]
  fn delete_course(courseAddress: Address) -> ZomeApiResult<()> {
    course::delete(&courseAddress)
  }

  #[zome_fn("hc_public")]
  fn get_courses() -> ZomeApiResult<Address> {
    course::list()
  }

  /**************************** Module Publish Function From Zome to OutSide */
  ////
  /// 
  /// 
  /// 
  /// 
  /**************************** Content Publish Function From Zome to OutSide */
  ///
  /// 
  /// 
  /// 
  /// 
}
