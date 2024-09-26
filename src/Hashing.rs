use hex_literal::hex;
use sha2:: {Digest, Sha256};
fn main(){
    let x : &str = "password";
    let mut hasher = Sha256::new();
    hasher. update (x);
    let result : Output<> = hasher. finalize();
    assert_eq! (result[..], hex!("5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"))|
    printin! ("The hash of the password is: {:?}", result);
}