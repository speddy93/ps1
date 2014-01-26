use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile> <inputfile2>" , args[0]); 
    } else {
        let fname1 = args[1].clone();
        let fname2 = args[2].clone();
        let path1 = Path::new(fname1.clone());
        let path2 = Path::new(fname2.clone());
        let msg1_file = File::open(&path1);
        let msg2_file = File::open(&path2);

        match (msg1_file,msg2_file) {
            (Some(mut msg1),Some(mut msg2)) => {
                let msg1_bytes: ~[u8] = msg1.read_to_end();
                let msg2_bytes: ~[u8] = msg2.read_to_end();
                let joined_file= File::create(&Path::new("joined.txt"));
                
                match (joined_file) {
                    Some(joined) => { 
                        join(msg1_bytes, msg2_bytes, joined); 
                        } ,
                    None => fail!("Error opening output files!"),
                }
            } ,
            (_,_) => fail!("Error opening message file: {:s}", fname1)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}
fn join(msg1_bytes: &[u8], msg2_bytes: &[u8], mut joined: File)
{
    let random_bytes=xor(msg1_bytes, msg2_bytes);
    joined.write(random_bytes);
    
}