/************************ Import Required Libraries */
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult, AGENT_ADDRESS};

use crate::course;
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
            match validation_data{
                EntryValidationData::Create{entry,validation_data} =>{
                let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();
                 match course::is_user_course_owner(&chain_entries,&entry.course_address){
                     Ok(_)=> validate_module_title(&entry.title),
                     Err(e)=> Err(e)
                 }
                },
                EntryValidationData::Modify{new_entry,old_entry,validation_data,..}=>{
                                    let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();
                    match course::is_user_course_owner(&chain_entries,&old_entry.course_address){
                     Ok(_)=> validate_module_title(&new_entry.title),
                     Err(e)=> Err(e)
                 }
                },
                EntryValidationData::Delete{old_entry,validation_data,..}=>{
                    let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();
                    match course::is_user_course_owner(&chain_entries,&old_entry.course_address){
                     Err(e)=> Err(e),
                     _=>Ok(())
                }

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
    let new_moduel = Module::new(title, course_address.clone());
    let new_module_entry = Entry::App("module".into(), new_moduel.into());
    let new_module_address = hdk::commit_entry(&new_module_entry)?;
    Ok(new_module_address)
}

pub fn update(title: String, module_address: &Address) -> Result<Address, String> {
    let current_module_json = hdk::get_entry(module_address).unwrap().unwrap();
    if let Entry::App(_, current_module_json) = current_module_json {
        let current_moduel =
            Module::try_from(current_module_json).expect("Entry at this address is not Course");
        let new_version_module = Module::new(title, current_moduel.course_address);
        let new_version_module_entry = Entry::App("module".into(), new_version_module.into());
        let edited_module_address =
            hdk::api::update_entry(new_version_module_entry, module_address)?;
        Ok(edited_module_address)
    } else {
        Err("Course has not found!".into())
    }
}

pub fn delete(module_address: &Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(module_address)
}
