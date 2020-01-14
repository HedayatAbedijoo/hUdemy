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

use hdk::holochain_core_types::{
    dna::entry_types::Sharing, entry::Entry, validation::EntryValidationData,
};
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult, AGENT_ADDRESS};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    Title: String,
    modules: Vec<Address>,
    teacher_address: Address,
}

impl Course {
    // Constrcuctor
    pub fn new(title: String, owner: Address) -> Self {
        Course {
            Title: title,
            teacher_address: owner,
        }
    }
}

////////////////////Course Entry Definition
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: courseEntry.into(),
        description: "this is the definition of course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Course>| {
            match _validation_data{
                EntryValidationData::Create{entry,validation_data:_} =>{ // validation on Create
                   validate_course_title(&_validation_data.Title)
                },
                EntryValidationData::Modify{old_etnry,new_entry,..}=>{
                    match validate_course_ownership(&old_etnry.teacher_address) = result {
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

// This is constant variable for Anchor.
const CourseAnchor: String = "course_list".to_string();
const courseEntry: String = "course".to_string();
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
                courseEntry.into(),
                link_type: CourseAnchor,
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
fn validate_course_title(title: &str) -> Result<(), Err> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

fn validate_course_ownership(courseOwnerAddress: &Address) -> Result<(), Err> {
    if courseOwnerAddress != AGENT_ADDRESS.to_string() {
        Err("You are not the owner of the Entry. So you can not change it.")
    } else {
        Ok(())
    }
}
/********************************************** */
/// Course Helper Functions: CRUD

pub fn create(title: String, teacherAddress: Address) -> ZomeApiResult<Address> {
    let new_course = Course {
        Title: title,
        teacher_address: teacherAddress,
    };
    let new_course_entry = Entry::App(courseEntry.into(), new_course.into());
    let new_course_address = hdk::commit_entry(&new_course_entry)?;

    Ok(new_course_address)
}

pub fn update(title: String, course_address: Address) -> ZomeApiResult<Address, ZomeApiError> {
    let current_course_JSON = hdk::get_entry(&course_address).unwrap().unwrap();
    if let Entry::App(_, current_course) = current_course {
        let current_course =
            Course::try_from(current_course).expect("Entry at this address is not Course");
        let new_version_course = Course::new(&title, &current_course.teacher_address);
        let edited_course_address = hdk::commit_entry(&new_version_course)?;
        Ok(edited_course_address)
    } else {
        Err("Course has not found!")
    }
}

pub fn delete(address: Address) -> ZomeApiResult<(), ZomeApiError> {
    // delete course
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    // Get list of the courses using    Anchor: Courses
}
