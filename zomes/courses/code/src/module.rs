/************************ Import Required Libraries */
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult, AGENT_ADDRESS};

use crate::{course::Course, validation};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Module {
    title: String,
    course_address: Address,
}

impl Module {
    pub fn new(title: String, course_address: Address) -> Self {
        Module {
            title: title,
            course_address: course_address,
            // content:Vec::default()
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("module".into(), self.into())
    }
}

/*********************** Course Validations */
fn validate_module_title(title: &str) -> Result<(), String> {
    if title.len() > 200 {
        Err("Module title is too long".into())
    } else {
        Ok(())
    }
}

// Entry Definition
pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "module",
        description: "this is the definition of module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: | validation_data: hdk::EntryValidationData<Module>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_module_title(&entry.title)?;
                    let course: Course = hdk::utils::get_as_type(entry.course_address.clone())?;
                    let agent_address = &validation_data.sources()[0];
                    if agent_address!=&course.teacher_address {
                                      return Err(String::from("Only the teacher can create a module for it"));
                                  }
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validate_module_title(&new_entry.title)?;

                    if new_entry.course_address != old_entry.course_address {
                        return Err(String::from("Cannot modify the course of a module"));
                    }
                    let course: Course = hdk::utils::get_as_type(new_entry.course_address.clone())?;

                    let agent_address = &validation_data.sources()[0];
                    if agent_address!=&course.teacher_address {
                                      return Err(String::from("Only the teacher can modify a module for it"));
                                  }

                    Ok(())
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    let course: Course = hdk::utils::get_as_type(old_entry.course_address.clone())?;
                    let agent_address = &validation_data.sources()[0];
                    if agent_address!= &course.teacher_address {
                                      return Err(String::from("Only the teacher can delete a module"));
                                  }

                    Ok(())
                }
            }
        },
        links:[
            to!(
                "content",
                link_type: "content_list",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}

pub fn create(title: String, course_address: &Address) -> ZomeApiResult<Address> {
    let mut course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_module = Module::new(title, course_address.clone());
    let new_module_address = hdk::commit_entry(&new_module.entry())?;

    course.modules.push(new_module_address.clone());

    hdk::update_entry(course.entry(), &course_address)?;

    Ok(new_module_address)
}

pub fn update(title: String, module_address: &Address) -> ZomeApiResult<Address> {
    let mut module: Module = hdk::utils::get_as_type(module_address.clone())?;

    module.title = title;

    hdk::update_entry(module.entry(), module_address)
}

pub fn delete(module_address: Address) -> ZomeApiResult<()> {
    let module: Module = hdk::utils::get_as_type(module_address.clone())?;

    let mut course: Course = hdk::utils::get_as_type(module.course_address.clone())?;

    hdk::remove_entry(&module_address)?;

    course.modules.remove_item(&module_address);

    hdk::update_entry(course.entry(), &module.course_address)?;

    Ok(())
}
