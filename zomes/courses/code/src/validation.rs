use hdk::prelude::*;

use crate::course::Course;
use crate::utils;

pub fn validate_author_is_teacher_of_course(
    chain_entries: &Vec<Entry>,
    course_address: &Address,
) -> ZomeApiResult<bool> {
    let agent_address = utils::get_chain_agent_id(chain_entries)?;
    let maybe_course: Option<Course> =
        utils::find_entry_with_address(chain_entries, "course", course_address)?;

    match maybe_course {
        Some(course) => Ok(course.teacher_address == agent_address),
        None => Err(ZomeApiError::from(String::from(
            "Course not found in the given source chain",
        ))),
    }
}

pub fn validate_chain_author(
    teacher_address: &Address,
    chain_entries: &Vec<Entry>,
) -> ZomeApiResult<()> {
    let agent_address = utils::get_chain_agent_id(&chain_entries)?;
    if agent_address != teacher_address.clone() {
        return Err(ZomeApiError::from(String::from(
            "Only the teacher can modify or delete the course",
        )));
    }
    Ok(())
}
