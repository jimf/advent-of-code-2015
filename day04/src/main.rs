extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

// Solution adapted from https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
fn main() {
    let secret_key = "iwrupvqb".as_bytes();
    let mut hasher = Md5::new();
    let mut first_five = 0;
    let mut first_six = 0;

    for i in 0..std::u64::MAX {
        hasher.input(secret_key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        if output[..2] == [0, 0] {
            // Starts with "0000"
            if output[2] == 0 && first_six == 0 {
                first_six = i;
            }
            if output[2] <= 0x0F && first_five == 0 {
                first_five = i;
            }
            if first_five != 0 && first_six != 0 {
                break;
            }
        }

        hasher.reset();
    }

    println!("A: {}", first_five);
    println!("B: {}", first_six);
}
