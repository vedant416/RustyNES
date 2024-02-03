use crate::mappers::Mapper;

pub struct Cart {
    pub data: CartData,
    pub mapper: Box<dyn Mapper>,
}


pub struct CartData {
    pub bytes: Vec<u8>,
}