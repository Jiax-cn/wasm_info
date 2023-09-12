use wasmparser::{
    NameSectionReader, Name, SectionLimited, Naming, Chunk, Payload, Parser,
};

pub fn parse_name_sec(wasm_blob: &[u8]) -> Option<SectionLimited<Naming>> {
    let mut cur_wasm = wasm_blob;
    let mut parser = Parser::new(0);

    loop {
        let (payload, consumed) = match parser.parse(cur_wasm, true).ok()? {
            Chunk::NeedMoreData(hint) => {
                panic!("Invalid Wasm module {:?}", hint);
            },
            Chunk::Parsed { consumed, payload } => (payload, consumed),
        };

        match payload {
            Payload::CustomSection(custom_sec) => {
                if custom_sec.name() == "name" {
                    let name_sec_reader = NameSectionReader::new(&custom_sec.data(), 0);
                    for item in name_sec_reader {
                        if let Ok(Name::Function(map)) = item {
                            return Some(map);
                        }
                    }
                }
            },
            Payload::End(_) => break,
            // todo: parse other section
            _ => {},
        }

        cur_wasm = &cur_wasm[consumed..];
    }

    None
}
