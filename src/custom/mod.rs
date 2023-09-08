use crate::common::{
    SectionId, Module,
};

use wasmparser::{
    NameSectionReader, Name, SectionLimited, Naming
};

pub fn get_name_map(module: &Module) -> Option<SectionLimited<Naming>> {
    if let Some(custom_section) = module.raw_sections.get(&SectionId::Custom.into()){
        let name_sec_reader = NameSectionReader::new(&custom_section.data, 0);
        for item in name_sec_reader {
            if let Ok(Name::Function(map)) = item {
                return Some(map);
            }
        }
    }
    None
}
