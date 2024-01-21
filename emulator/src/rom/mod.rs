use crate::mappers::Mapper;
// divide Cart into data and mapper,
// so that we can mutably borrow the "data" and "mapper" separately
pub struct Cart {
    pub data: CartData,
    pub mapper: Box<dyn Mapper>,
}


pub struct CartData {
    pub bytes: Vec<u8>,
}