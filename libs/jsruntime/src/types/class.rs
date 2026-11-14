use jsgc::HandleMut;
use jsgc::Seq;
use jsparser::Symbol;

use super::Object;

#[derive(Debug)]
#[repr(C)]
pub struct ClassElement {
    pub value: Option<HandleMut<Object>>,
    pub key: Symbol,
    pub kind: ClassElementKind,
}

base::static_assert_eq!(ClassElement::SIZE, 16); // TODO: 32bit arch
base::static_assert_eq!(ClassElement::ALIGNMENT, 8); // TODO: 32bit arch

impl ClassElement {
    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const VALUE_OFFSET: usize = std::mem::offset_of!(Self, value);
    pub(crate) const KEY_OFFSET: usize = std::mem::offset_of!(Self, key);
    pub(crate) const KIND_OFFSET: usize = std::mem::offset_of!(Self, kind);
}

#[repr(u8)]
#[derive(Debug)]
pub enum ClassElementKind {
    Constructor,
    StaticField,
    StaticBlock,
    StaticMethod,
    Field,
    Method,
}

#[derive(Debug, jsgc_derive::Trace)]
pub struct ClassInitializer {
    fields: Seq<ClassFieldDefinition>,
}

impl ClassInitializer {
    pub fn new(fields: Seq<ClassFieldDefinition>) -> Self {
        Self { fields }
    }

    pub fn fields(&self) -> &[ClassFieldDefinition] {
        self.fields.as_slice()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClassFieldDefinition {
    pub name: Symbol,
    pub initializer: Option<HandleMut<Object>>,
}

impl jsgc::Trace for ClassFieldDefinition {
    #[inline]
    fn trace(&self, visits: &mut jsgc::VisitList) {
        self.initializer.trace(visits);
    }
}
