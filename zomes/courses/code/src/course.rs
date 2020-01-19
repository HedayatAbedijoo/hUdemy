/************************ Import Required Libraries */
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use crate::hdk::prelude::AddressableContent;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use hdk::ValidationData;
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    title: String,
    modules: Vec<Address>,
    teacher_address: Address,
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
    pub fn new_with_moduels(title: String, owner: Address, modules: Vec<Address>) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: modules,
        }
    }
}

////////////////////Course Entry Definition
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is the definition of course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Course>| {
            match _validation_data{
                EntryValidationData::Create{entry,..} =>{
                   validate_course_title(&entry.title)
                },
                EntryValidationData::Modify{new_entry,old_entry,..}=>{
                    match validate_course_ownership(&old_entry.teacher_address) {
                        Ok(_)=> validate_course_title(&new_entry.title),
                        Err(e) => Err(e)
                    }
                },
                EntryValidationData::Delete{old_entry,..}=>{
                  validate_course_ownership(&old_entry.teacher_address)
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

fn validate_course_ownership(course_owner_address: &Address) -> Result<(), String> {
    if course_owner_address.to_string() != AGENT_ADDRESS.to_string() {
        Err("You are not the owner of the Entry. So you can not change it.".into())
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

pub fn update(title: String, course_address: &Address) -> Result<Address, String> {
    let current_course_json = hdk::get_entry(&course_address).unwrap().unwrap();
    if let Entry::App(_, current_course_json) = current_course_json {
        let current_course =
            Course::try_from(current_course_json).expect("Entry at this address is not Course");
        let new_version_course = Course::new_with_moduels(
            title,
            current_course.teacher_address,
            current_course.modules,
        );
        let new_version_course_entry = Entry::App("course".into(), new_version_course.into());
        let edited_course_address =
            hdk::api::update_entry(new_version_course_entry, course_address)?;
        Ok(edited_course_address)
    } else {
        Err("Course has not found!".into())
    }
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

pub fn is_user_course_owner(
    validation_data: ValidationData,
    course_address: &Address,
) -> Result<(), String> {
    let source_chain = validation_data
        .package
        .source_chain_entries
        .ok_or("Could not retrieve source chain")?;
    let course = source_chain
        .iter()
        .filter(|entry| entry.address() == course_address.to_owned())
        .filter_map(|entry| {
            if let Entry::App(_, entry_data) = entry {
                Some(Course::try_from(entry_data.clone()).unwrap())
            } else {
                None
            }
        })
        .next()
        .ok_or(ZomeApiError::HashNotFound)?;

    validate_course_ownership(&course.teacher_address)
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
        hdk::api::update_entry(
            Entry::App("course".into(), course_entry.into()),
            course_address,
        )
    } else {
        panic!("This address is not a valid address")
    }
}

pub fn update_module_in_course(
    course_address: &Address,
    old_module_address: &Address,
    new_module_address: &Address,
) -> ZomeApiResult<Address> {
    let current_course = hdk::get_entry(course_address).unwrap().unwrap();
    if let Entry::App(_, current_course) = current_course {
        let mut course_entry = Course::try_from(current_course.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");
        // remove the old address from Modules,
        let index = course_entry
            .modules
            .iter()
            .position(|x| x == old_module_address)
            .unwrap();
        course_entry.modules.remove(index);
        // add new module address to list of Modules
        course_entry.modules.push(new_module_address.clone());
        hdk::api::update_entry(
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
