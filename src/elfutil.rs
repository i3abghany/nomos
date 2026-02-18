use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

pub fn get_text_section(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_data = fs::read(path)?;
    let obj_file = object::File::parse(&*file_data)?;
    let text_section = obj_file
        .section_by_name(".text")
        .ok_or("No .text section")?;
    let data = text_section.data()?;
    Ok(data.to_vec())
}
