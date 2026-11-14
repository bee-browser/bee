//! DWARF debug information generator module using the `gimli` crate.
//!
//! This module constructs DWARF debug sections and compilation units for JIT-compiled functions.

use gimli::Encoding;
use gimli::Format;
use gimli::LittleEndian;
use gimli::write::Address;
use gimli::write::AttributeValue;
use gimli::write::DwarfUnit;
use gimli::write::EndianVec;
use gimli::write::Sections;

use jsruntime::LambdaId;

/// Generates DWARF debug sections from the given function info.
///
/// This function constructs the DWARF debugging information entries (DIEs)
/// for the specified JIT function, mapping its address range and symbol name.
pub fn generate_dwarf(
    id: LambdaId,
    code: &[u8],
    code_load_addr: usize,
) -> Sections<EndianVec<LittleEndian>> {
    let symbol_name = id.to_string();

    let encoding = Encoding {
        format: Format::Dwarf32,
        version: 4,
        address_size: 8, // x86_64
    };

    let mut dwarf = DwarfUnit::new(encoding);

    let symbol_address = Address::Constant(code_load_addr as u64);
    let root_idx = dwarf.unit.root();

    let entry = dwarf.unit.get_mut(root_idx);
    entry.set(
        gimli::DW_AT_name,
        AttributeValue::String(symbol_name.as_bytes().into()),
    );
    entry.set(gimli::DW_AT_low_pc, AttributeValue::Address(symbol_address));
    entry.set(
        gimli::DW_AT_high_pc,
        AttributeValue::Udata(code.len() as u64),
    );

    let subprogram_entry = dwarf.unit.add(root_idx, gimli::DW_TAG_subprogram);
    let sub_entry = dwarf.unit.get_mut(subprogram_entry);
    sub_entry.set(
        gimli::DW_AT_name,
        AttributeValue::String(symbol_name.as_bytes().into()),
    );
    sub_entry.set(gimli::DW_AT_low_pc, AttributeValue::Address(symbol_address));
    sub_entry.set(
        gimli::DW_AT_high_pc,
        AttributeValue::Udata(code.len() as u64),
    );

    let mut dwarf_sections = Sections::new(EndianVec::new(LittleEndian));
    dwarf.write(&mut dwarf_sections).unwrap();

    dwarf_sections
}
