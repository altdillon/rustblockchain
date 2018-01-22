extern crate blockchain;
extern crate sha2;

use blockchain::block;
use sha2::{Sha256, Digest};

fn main()
{
    block::hello();
    let mut lasthash= Sha256::default();

    let test_block = block::BlockElem::new(0,&lasthash);
}

fn testhash()
{
    let mut hasher = Sha256::default();
    let text= b"hello world!";
    hasher.input(text);
    let output = hasher.result();

    for i in output.iter()
    {
        println!("{}",i);
    }
}
