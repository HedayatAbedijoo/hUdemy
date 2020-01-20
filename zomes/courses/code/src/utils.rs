use hdk::prelude::*;
use std::convert::TryFrom;

pub fn find_entry_with_address<T>(
    chain_entries: &Vec<Entry>,
    entry_type: &str,
    address: &Address,
) -> ZomeApiResult<Option<T>>
where
    T: TryFrom<JsonString> + Clone,
{
    for entry in chain_entries {
        if let Entry::App(current_entry_type, entry_content) = entry {
            let entry_address = hdk::entry_address(&entry)?;
            if current_entry_type.to_string() == entry_type && entry_address == address.clone() {
                let content = T::try_from(entry_content.clone());

                if let Ok(c) = content {
                    return Ok(Some(c));
                } else {
                    return Err(ZomeApiError::from(String::from("Error converting entry")));
                }
            }
        }
    }

    Ok(None)
}
