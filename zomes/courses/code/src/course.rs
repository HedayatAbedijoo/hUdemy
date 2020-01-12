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
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
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
}


let CourseAnchor: String   = "Courses".to_string();

////////////////////Entry Definition
/// Entry Link
     #[entry_def]
     fn Course_def() -> ValidatingEntryType {
        entry!(
            name: "Course",
            description: "this is the definition of course",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Course>| {
               
               //// if course title is more than 50 charater throw an Error
                Ok(())
            }
        )
    }

////////////////////
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