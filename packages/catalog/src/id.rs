use uuid::Uuid;

pub type ProductId=Uuid;

pub trait IdGenerator:Send+Sync{
    fn new_id(&self)->ProductId;
}

pub struct UuidV7Generator;

impl IdGenerator for UuidV7Generator{

    fn new_id(&self)->ProductId {
      Uuid::now_v7()
    }
}

pub struct FixedIdGenerator(pub ProductId);

impl IdGenerator for FixedIdGenerator {
    fn new_id(&self) -> ProductId {
        self.0
    }
}