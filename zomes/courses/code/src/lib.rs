/***************** Required Library */
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

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

//use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */
mod course;
mod module;
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
    course::create(&title)
  }

  #[zome_fn("hc_public")]
  fn update_course(title: String, course_address: Address) -> Result<Address, String> {
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

  /**************************** Module Entry Definition & Functions */
  #[entry_def]
  fn module_entry_definition() -> ValidatingEntryType {
    module::entry_def()
  }

  #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
  pub struct ModuleResultOperation {
    module_address: Address,
    course_address: Address,
  }
  #[zome_fn("hc_public")]
  fn create_module(title: String, course_address: Address) -> ZomeApiResult<ModuleResultOperation> {
    let module_address = module::create(title, &course_address)?;
    let updated_course_address = course::add_module_to_course(&course_address, &module_address)?;
    let result = ModuleResultOperation {
      module_address: module_address,
      course_address: updated_course_address,
    };
    Ok(result)
  }

  #[zome_fn("hc_public")]
  fn update_module(
    title: String,
    module_address: Address,
    course_address: Address,
  ) -> ZomeApiResult<ModuleResultOperation> {
    let updated_module_address = module::update(title, &module_address)?;
    let updated_course_address =
      course::update_module_in_course(&course_address, &module_address, &updated_module_address)?;
    let result = ModuleResultOperation {
      module_address: updated_module_address,
      course_address: updated_course_address,
    };
    Ok(result)
  }

  #[zome_fn("hc_public")]
  fn delete_module(
    module_address: Address,
    course_address: Address,
  ) -> ZomeApiResult<ModuleResultOperation> {
    let _address = module::delete(&module_address);
    let updated_course_address =
      course::remove_module_from_course(&course_address, &module_address)?;
    let result = ModuleResultOperation {
      module_address: module_address,
      course_address: updated_course_address,
    };
    Ok(result)
  }

  /**************************** Content Zome Functions */
}
