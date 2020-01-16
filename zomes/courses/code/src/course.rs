/************************ Import Required Libraries */
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult, ZomeApiError},    
    AGENT_ADDRESS,
};

use hdk::holochain_core_types::{    
    validation::EntryValidationData,
    entry::Entry    
};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use hdk::holochain_json_api::{json::JsonString, error::JsonError};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use std::convert::TryFrom;
use crate::hdk::prelude::AddressableContent;
use hdk::ValidationData;
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

pub fn create(title: &str) -> ZomeApiResult<Address> {  
    let new_course = Course::new(title.into(),AGENT_ADDRESS.to_string().into());
    let new_course_entry = Entry::App("course".into(), new_course.into());
    let new_course_address = hdk::commit_entry(&new_course_entry)?;

    Ok(new_course_address)
}

pub fn update(title: String, course_address: &Address) -> Result<Address,String> {
    let current_course_json = hdk::get_entry(&course_address).unwrap().unwrap();
    if let Entry::App(_, current_course_json) = current_course_json {
        let current_course =
            Course::try_from(current_course_json).expect("Entry at this address is not Course");
        let new_version_course = Course::new(title, current_course.teacher_address);        
        let edited_course_address = hdk::commit_entry(&Entry::App("course".into(),new_version_course.into()))?;
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



pub fn is_user_course_owner(validation_data: ValidationData, course_address:&Address)-> Result<(), String>{

let local_chain = validation_data.package.source_chain_entries
                		.ok_or("Could not retrieve source chain")?;
let course = local_chain
        .iter()
        .filter(|entry| {
            entry.address() == course_address.to_owned()
        })
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