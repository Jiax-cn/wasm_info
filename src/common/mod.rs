use wasmparser::{
    Parser, Chunk, Payload,
};

use alloc::{
	collections::{BTreeMap},
	vec::Vec,
};

use anyhow::{Result};

use core::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
    Tag = 13,
}

impl From<SectionId> for u8 {
    #[inline]
    fn from(id: SectionId) -> u8 {
        id as u8
    }
}

#[derive(Clone, Debug)]
pub struct RawSection {
	/// The id for this section.
	pub id: u8,
	/// The raw data for this section.
	pub data: Vec<u8>,
}

impl RawSection {
	pub fn new(id: u8, data: Vec<u8>) -> Self {
		RawSection { id, data }
	}
}

#[derive(Default)]
pub struct Module{
	pub raw_sections: BTreeMap<u8, RawSection>,
}

impl Module {
    pub fn new(wasm_blob: &[u8]) -> Result<Module> {
        let mut cur_wasm = wasm_blob;
        let mut parser = Parser::new(0);
        let mut module = Module::default();

        loop{
            let (payload, consumed) = match parser.parse(cur_wasm, true)? {
				Chunk::NeedMoreData(hint) => {
					panic!("Invalid Wasm module {:?}", hint);
				},
				Chunk::Parsed { consumed, payload } => (payload, consumed),
            };

            match payload {
                Payload::CustomSection(custom_sec) => {
                    match custom_sec.name() {
                        "name" =>  {
                            module.section(
                                SectionId::Custom.into(),
                                Range { start: custom_sec.data_offset(), end: custom_sec.range().end },
                                wasm_blob,
                            );
                        },
                        // todo: parse tother custom section
                        _ => {},
                    }
                },
                Payload::End(_) => break,
                // todo: parse tother section
                _ => {},
            }

			cur_wasm = &cur_wasm[consumed..];
        }
        Ok(module)
    }

    fn section(&mut self, id: u8, range: Range<usize>, full_wasm: &[u8]) {
		self.raw_sections.insert(id, RawSection::new(id, full_wasm[range].to_vec()));
	}
}