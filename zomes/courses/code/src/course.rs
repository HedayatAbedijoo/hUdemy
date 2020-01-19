/************************ Import Required Libraries */
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use crate::{utils, validation};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::AddressableContent;
use hdk::prelude::LinkMatch;
use hdk::ValidationData;
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    pub title: String,
    pub modules: Vec<Address>,
    pub teacher_address: Address,
}

impl Course {
    // Constrcuctor
    pub fn new(title: String, owner: Address) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: Vec::default(),
        }
    }
    pub fn from(title: String, owner: Address, modules: Vec<Address>) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: modules,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("course".into(), self.into())
    }
}

////////////////////Course Entry Definition
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is the definition of course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { entry, .. } => {
                   validate_course_title(&entry.title)
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validate_course_title(&new_entry.title)?;

                    if new_entry.teacher_address != old_entry.teacher_address {
                        return Err(String::from("Cannot change the teacher of the course"));
                    }

                    let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();

                    validation::validate_chain_author(&old_entry.teacher_address, &chain_entries)?;

                    Ok(())
                },
                EntryValidationData::Delete {old_entry, validation_data, .. } => {
                    let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();

                    validation::validate_chain_author(&old_entry.teacher_address, &chain_entries)?;

                    Ok(())
                }
            }
        }
    )
}

//// Anchor Definition : This Anchor will be used to query all courses
pub fn anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description:"Anchor to all Courses",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<String>|{
            Ok(())
        },
        links:[
            to!(
                "course",
                link_type: "course_list",
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
/*********************** Course Validations */
fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

/********************************************** */
/// Course Helper Functions: CRUD

pub fn create(title: String) -> ZomeApiResult<Address> {
    let new_course = Course::new(title, AGENT_ADDRESS.to_string().into());
    let new_course_entry = Entry::App("course".into(), new_course.into());
    let new_course_address = hdk::commit_entry(&new_course_entry)?;

    Ok(new_course_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    let course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_version_course = Course::from(title, course.teacher_address, modules_addresses);
    let new_version_course_entry = Entry::App("course".into(), new_version_course.into());

    hdk::update_entry(new_version_course_entry, course_address)
}

pub fn delete(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&address)
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    let anchor_entry = Entry::App("course_list".into(), "course".into());
    let anchor_address = hdk::commit_entry(&anchor_entry)?; // if Anchor exist, it returns the commited one.
    let addresses = hdk::get_links(
        &anchor_address,
        LinkMatch::Exactly("course_list"),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

pub fn add_module_to_course(
    course_address: &Address,
    module_address: &Address,
) -> ZomeApiResult<Address> {
    let current_course = hdk::get_entry(course_address).unwrap().unwrap();
    if let Entry::App(_, current_course) = current_course {
        let mut course_entry = Course::try_from(current_course.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");
        course_entry.modules.push(module_address.clone());
        hdk::update_entry(
            Entry::App("course".into(), course_entry.into()),
            course_address,
        )
    } else {
        panic!("This address is not a valid address")
    }
}

pub fn remove_module_from_course(
    course_address: &Address,
    module_address: &Address,
) -> ZomeApiResult<Address> {
    let current_course = hdk::get_entry(course_address).unwrap().unwrap();
    if let Entry::App(_, current_course) = current_course {
        let mut course_entry = Course::try_from(current_course.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");

        // remove the old address from Modules,
        let index = course_entry
            .modules
            .iter()
            .position(|x| x == module_address)
            .unwrap();
        course_entry.modules.remove(index);

        hdk::api::update_entry(
            Entry::App("course".into(), course_entry.into()),
            course_address,
        )
    } else {
        panic!("This address is not a valid address")
    }
}
