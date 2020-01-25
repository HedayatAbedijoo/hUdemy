/***************** Required Library */
#![feature(vec_remove_item)]
#![allow(dead_code)]
#![allow(unused_imports)]
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

use hdk::prelude::*;

//use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */
mod content;
mod course;
mod module;
use course::Course;
mod helper;
#[zome]
mod course_zome {

  #[init]
  fn init() {
    Ok(())
  }

  #[validate_agent]
  pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
    Ok(())
  }

  /**************************** Course Entry Definition and Functions */
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
    course::create(title)
  }

  #[zome_fn("hc_public")]
  fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
  }

  #[zome_fn("hc_public")]
  fn update_course(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: Address,
  ) -> ZomeApiResult<Address> {
    course::update(title, modules_addresses, &course_address)
  }

  #[zome_fn("hc_public")]
  fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
    course::delete(course_address)
  }

  #[zome_fn("hc_public")]
  fn get_courses() -> ZomeApiResult<Vec<Address>> {
    course::list()
  }
  #[zome_fn("hc_public")]
  fn get_my_courses() -> ZomeApiResult<Vec<ZomeApiResult<GetEntryResult>>> {
    course::get_my_courses()
  }
  /**************************** Module Entry Definition & Functions */
  #[entry_def]
  fn module_entry_definition() -> ValidatingEntryType {
    module::entry_def()
  }

  #[zome_fn("hc_public")]
  fn create_module(title: String, course_address: Address) -> ZomeApiResult<Address> {
    module::create(title, &course_address)
  }

  #[zome_fn("hc_public")]
  fn update_module_title(title: String, module_address: Address) -> ZomeApiResult<Address> {
    module::update(title, &module_address)
  }

  #[zome_fn("hc_public")]
  fn delete_module(module_address: Address) -> ZomeApiResult<()> {
    module::delete(module_address)
  }

  /**************************** Content Zome Functions */
}
