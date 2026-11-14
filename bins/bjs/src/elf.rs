//! ELF container builder module using the `object` crate.
//!
//! This module packages machine code and DWARF debug sections into an in-memory ELF binary.

use jsruntime::LambdaId;

/// Builds an in-memory ELF binary containing the machine code and DWARF sections.
///
/// This function creates an ELF container for the specified JIT-compiled function,
/// integrating both the executable text section and associated debug information.
pub fn build_jit_elf(id: LambdaId, code: &[u8], code_load_addr: usize) -> Vec<u8> {
    let symbol_name = id.to_string();

    // Generate DWARF debug sections from the dwarf module.
    let dwarf_sections = crate::dwarf::generate_dwarf(id, code, code_load_addr);

    let mut obj = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::X86_64,
        object::Endianness::Little,
    );

    // Add the .text section for actual machine code.
    let text_section = obj.add_section(
        b".text".to_vec(),
        b".text".to_vec(),
        object::SectionKind::Text,
    );
    obj.append_section_data(text_section, code, 1);

    // Register the symbol in the symbol table.
    obj.add_symbol(object::write::Symbol {
        name: symbol_name.into_bytes(),
        value: code_load_addr as u64,
        size: code.len() as u64,
        kind: object::SymbolKind::Text,
        scope: object::SymbolScope::Dynamic,
        weak: false,
        section: object::write::SymbolSection::Section(text_section),
        flags: object::SymbolFlags::None,
    });

    // Merge DWARF sections into the ELF object.
    dwarf_sections
        .for_each(|section_id, data| {
            let name = section_id.name().as_bytes().to_vec();
            let kind = match section_id {
                gimli::SectionId::DebugInfo
                | gimli::SectionId::DebugAbbrev
                | gimli::SectionId::DebugStr
                | gimli::SectionId::DebugLine => object::SectionKind::Debug,
                _ => object::SectionKind::Other,
            };
            let sec = obj.add_section(name.clone(), name, kind);
            obj.append_section_data(sec, data.slice(), 1);
            Ok::<(), ()>(())
        })
        .unwrap();

    obj.write().unwrap()
}
