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
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    validation::EntryValidationData
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;


#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Course {
    Title: String,
    modules:Vec!<Address>,
    teacher_address: Address
}

impl Course{
    // Constrcuctor
    pub fn new(title:String, owner:Address)-> Self{
        Course{
            Title:title,
            teacher_address:owner
        }
    }

    
    
}

// This is constant variable for Anchor. We will use this Anchor to query all courses
let CourseAnchor: String   = "Courses".to_string();

////////////////////Entry Definition
   pub fn Course_def() -> ValidatingEntryType {
        entry!(
            name: "Course",
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

//////////////////// Course Entry Validation
/// 
/// 

fn validate_course_title(title:&str)->Result<(),Err>{

    if title.len()>50 {
        Err("Course title is too long".into())
    }
    else{
        Ok(())
    }

}

fn validate_course_ownership(courseOwnerAddress:&Address)-> Result<(),Err>{
    if courseOwnerAddress != AGENT_ADDRESS.to_string(){
        Err("You are not the owner of the Entry. So you can not change it.")
    } else{
        Ok(())
    }
}

/// Entry Helper Functions: CRUD
/// 
   
     #[zome_fn("hc_public")]
    fn create(title:String, teacherAddress:Address) -> ZomeApiResult<Address> {
     /// 1 Create Course
     /// 2 Create Course_Teacher link
     /// 3 Create Teacher_Course link 
     /// 3 Create "Courses" Anchor
    }

     #[zome_fn("hc_public")]
    fn update(title:String, address:Address) -> ZomeApiResult<()),ZomeApiError> {
     // Update Entry
     // update link 
    }


     #[zome_fn("hc_public")]
    fn delete(address:Address) -> ZomeApiResult<(), ZomeApiError> {
     // delete course
    }

     #[zome_fn("hc_public")]
    fn list() -> ZomeApiResult<Vec!<Address>> {
     // Get list of the courses using    Anchor: Courses
    }