/***************** Required Library */
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
    error::ZomeApiResult
};

//use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */
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
  fn update_course(title: String, course_address: Address) -> Result<Address,String> {
    course::update(title, &course_address)
  }

  #[zome_fn("hc_public")]
  fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
    course::delete(course_address)
  }

  #[zome_fn("hc_public")]
  fn get_courses() -> ZomeApiResult<Vec<Address>> {
    course::list()
  }

  /**************************** Module Publish Function From Zome to OutSide */

  /**************************** Content Publish Function From Zome to OutSide */

}
