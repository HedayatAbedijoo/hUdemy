use hdk::prelude::*;

use crate::course::Course;
use crate::utils;

pub fn validate_teacher_signed_module(
    chain_entries: &Vec<Entry>,
    signing_addresses: &Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<bool> {
    let maybe_course: Option<Course> =
        utils::find_entry_with_address(chain_entries, "course", course_address)?;

    match maybe_course {
        Some(course) => Ok(signing_addresses.contains(&course.teacher_address)),
        None => Err(ZomeApiError::from(String::from(
            "Course not found in the given source chain",
        ))),
    }
}
