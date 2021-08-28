use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        2568005,
        "Genesis block".to_owned(),
        0x0000fffffffffffffffffffffffffffff);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}
