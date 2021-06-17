use bstream::FooBlockRef;
use bstream::{Block, BlockRef, DefaultBlockReference};
use std::fmt::Debug;
use std::hash::Hash;

fn main() {
    let r = DefaultBlockReference::new(10, String::from("toto"));
    // let f = FooBlockRef {
    //     num: 0,
    //     hash: "".to_string(),
    // };

    // let _b = Block::new(100);

    // let b2: Box<dyn BlockRef> = Box::new(r);
    // let my_box = Box::new(r);
    display(r);
    // display(f);
}

fn display<B: BlockRef>(r: B) {
    println!("{:?}", r.hash());
}

fn display2(r: impl BlockRef + Debug) {
    println!("{:?}", r);
}

// fn display(r: Box<dyn BlockRef>) {
//     println!("{:?}", r.hash());
// }
