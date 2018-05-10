extern crate sodiumoxide;

use sodiumoxide;
use blockchain::Blockchain;


fn main() {
    sodiumoxide::init();
    let boringchain = Blockchain::new();


}
