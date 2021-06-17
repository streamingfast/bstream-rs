use std::fmt::Debug;

#[derive(Debug)]
pub struct Block {
    pub number: u64,
}

pub trait BlockRef: Debug {
    fn num(&self) -> u64;
    fn hash(&self) -> &String;
}

// impl std::fmt::Debug for dyn BlockRef {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(fmt, "num: {:?}, hash: {:?}", self.num(), self.hash())
//     }
// }

#[derive(Debug)]
pub struct DefaultBlockReference {
    pub number: u64,
    pub hash: String,
}

#[derive(Debug)]
pub struct FooBlockRef {
    pub number: u64,
    pub hash: String,
}

impl BlockRef for FooBlockRef {
    fn num(&self) -> u64 {
        todo!()
    }

    fn hash(&self) -> &String {
        todo!()
        // unimplemented!()
    }
}

impl Block {
    pub fn new(n: u64) -> Block {
        Block { number: n }
    }
}

impl DefaultBlockReference {
    pub fn new(n: u64, r: String) -> DefaultBlockReference {
        DefaultBlockReference { number: n, hash: r }
    }
}

impl BlockRef for DefaultBlockReference {
    fn num(&self) -> u64 {
        self.number
    }

    fn hash(&self) -> &String {
        &self.hash
    }
}
