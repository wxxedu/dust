use dust_core::Generator;

pub struct DartGenerator;

impl Generator for DartGenerator {
    fn generate_struct(&mut self, data: &dust_core::StructType) {
        todo!()
    }

    fn generate_enum(&mut self, data: &dust_core::EnumType) {
        todo!()
    }

    fn generate_fn(&mut self, data: &dust_core::FnType) {
        todo!()
    }

    fn generate_union(&mut self, data: &dust_core::UnionType) {
        todo!()
    }
}
