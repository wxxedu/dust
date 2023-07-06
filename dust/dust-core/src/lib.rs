/// The generator.
pub trait Generator {
    /// Generates a struct.
    fn generate_struct(&mut self, data: &StructType);

    /// Generates an enum.
    fn generate_enum(&mut self, data: &EnumType);

    /// Generates a function.
    fn generate_fn(&mut self, data: &FnType);

    /// Generates a union.
    fn generate_union(&mut self, data: &UnionType);
}

#[non_exhaustive]
pub enum RsType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    Str,
    String,
    Ptr(PtrType),
    Slice(SliceType),
    Vec(VecType),
    Custom(CustomType),
}

pub enum CustomType {
    Struct(StructType),
    Enum(EnumType),
    Fn(FnType),
    Union(UnionType),
}

pub struct VecType {}

pub struct PtrType {}

pub struct SliceType {}

pub struct StructType {}

pub struct EnumType {}

pub struct FnType {}

pub struct UnionType {}

pub struct StructData {}
