/************************ Import Required Libraries */
//#![allow(unused_imports)] 

use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult},
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
//mod crate::course;
use crate::course;

/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Module {
    title: String,
    //content:Vec<Address>,    
    course_address: Address
}

impl Module{
    pub fn new(title:String,course_address:Address)->Self{
        Module{
            title:title,
            course_address:course_address,
           // content:Vec::default()
        }
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
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: "module",
        description: "this is the definition of module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Module>| {                           
            match validation_data{
                EntryValidationData::Create{entry,validation_data} =>{
                 match course::is_user_course_owner(validation_data,&entry.course_address){
                     Ok(_)=> validate_module_title(&entry.title),
                     Err(e)=> Err(e)
                 }                                                    
                },
                EntryValidationData::Modify{new_entry,old_entry,validation_data,..}=>{                   
                    match course::is_user_course_owner(validation_data,&old_entry.course_address){
                     Ok(_)=> validate_module_title(&new_entry.title),
                     Err(e)=> Err(e)
                 } 
                },
                EntryValidationData::Delete{old_entry,validation_data,..}=>{                  
                    match course::is_user_course_owner(validation_data,&old_entry.course_address){                     
                     Err(e)=> Err(e),
                     _=>Ok(())
                }

                }
            }
        }
    )
}


// pub fn create(title:String,course_address:Address)-> ZomeApiResult<Address>{

// }

///// Helper Function
/*
pub fn add(title: string, course_address: Address) -> ZomeApiResult<Address, Error> {
    // Create Module entry
    // Create Course_Module link
    /// Retun Address 
}

pub fn update(title:String, moduel_address:Address) -> ZomeApiResult<(),ZomeApiError>){
    // update module with new title
}

pub fn delete(module_address::Address) -> ZomeApiResult<(),ZomeApiError>{
    // delete module
}

pub fn list(course_address:Address) -> ZomeApiResult<vec!<Address>>{

}
*/