

//fa 7a 3f 8c ca e6 b8 24 75 47 95 6a 0b 2a 43 cb b0 ee ed 05 59 d9 64 0f c9 58 1e d8 59 f6 aa f8
const PRIVATE_KEY: [u8;32] =[
   0xfa, 0x7a, 0x3f, 0x8c, 0xca, 0xe6, 0xb8, 0x24, 0x75, 0x47, 0x95, 0x6a, 0x0b, 0x2a, 0x43, 0xcb,
    0xb0, 0xee, 0xed, 0x05, 0x59, 0xd9, 0x64, 0x0f, 0xc9, 0x58, 0x1e, 0xd8, 0x59, 0xf6, 0xaa, 0xf8
];
//3b 43 09 32 1b 93 ba a7 50 7c 4f 16 2b 6e 38 7b cc 6a f2 13 0c f8 38 33 44 2f a0 0f 46 be e2 4c
const PUBLIC_KEY: [u8;32]=[
    0x3b, 0x43, 0x09, 0x32, 0x1b, 0x93, 0xba, 0xa7, 0x50, 0x7c, 0x4f, 0x16, 0x2b, 0x6e, 0x38, 0x7b,
    0xcc, 0x6a, 0xf2, 0x13, 0x0c, 0xf8, 0x38, 0x33, 0x44, 0x2f, 0xa0, 0x0f, 0x46, 0xbe, 0xe2, 0x4c
];

const ADDRESS:&str ="BQpPHJHL2yg3PDdJPXu6h8RFguEqnAH31x68ptTkYmXA";


#[cfg(test)]
mod test{
    use std::str::FromStr;
    use arrayref::mut_array_refs;
    use ed25519_dalek::ed25519::signature::SignerMut;
    use hex::ToHex;
    // use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
    use super::*;
    use rustc_hex::*;
    use solana_program::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signature, Signer};


    #[test]
    fn test_address_equip(){

        const prk:[u8;32]=[172,148,165,89,214,130,50,246,7,36,26,112,38,133,40,132,53,199,56,106,114,41,147,86,129,163,239,9,83,199,
            215,243];
        const ppk:[u8;32] =[65,165,6,66,68,62,201,59,151,117,222,39,143,34,173,6,223,55,196,36,150,167,29,226,64,189,192,162,119,130,188,97];
        let private_key = super::PRIVATE_KEY;
        let public_key = super::PUBLIC_KEY;
        let address = super::ADDRESS;
        let mut buffer = [0u8; 64];
        let  (
            secret,
            public

        )= mut_array_refs![&mut buffer, 32, 32];
        secret.copy_from_slice(&private_key);
        public.copy_from_slice(&public_key);
        println!("key :{:?}",buffer.as_slice());

         let mut sk = Keypair::from_bytes(&buffer).unwrap();
        // let pk = sk.verifying_key();
        // let vk =VerifyingKey::from_bytes(&public_key).unwrap();
        let signatures=    sk.sign_message(b"hello world");
        println!("signatures: {:?}", signatures.to_string());
         println!("sad{:?}", sk.to_base58_string());
        println!("address {:?}", sk.pubkey().to_string());
        assert!(signatures.verify(&sk.pubkey().to_bytes()[..], b"hello world"), "not ok");
      //
      //   assert!(vk.verify(b"hello world", &signatures).is_ok(),"not ok");
      //
      //   println!("sad{:?}", pk.to_bytes().to_hex::<String>());
       //
       // let kp =  Keypair::new();
       //  println!("keypair: {:?}", kp.secret().to_bytes().to_hex::<String>());
       //  println!("publick : {:?}", kp.pubkey().to_bytes().to_hex::<String>());
       // //
        // let expected_public = kp.;
        // let secretPublic = ed25519_dalek::PublicKey::from(&secret);
        // println!("Public key: {:?}", expected_public.as_bytes().to_hex::<String>());
        // println!("origin key: {:?}", secretPublic.as_bytes().to_hex::<String>());

       //  println!("Public key: {:?}", expected_public.as_ref().unwrap().as_bytes().to_hex::<String>());
       //  println!("origin key: {:?}", public_key.as_slice().to_hex::<String>())

    }


    #[test]
    fn test_signature_verify(){
        let addr = "5RFT2otTsBfPUJckGaDU5SPeCgrHonWVWgvMo2nVSZuJ";
        let signature = "2bEYEqz6kBcS8GiD271NrvipttMqoEHeW8NARoxP9Y6wjNvSRv5QFaRvwXpTfKXv3d7Xams86iHsiFH4EK9buQxF";
        let pk=  Pubkey::try_from(addr).unwrap();

       let sig=Signature::from_str(signature).unwrap();
        assert!(sig.verify(&pk.to_bytes()[..], b"hello world"));
       //  assert!(sig.verify(&pk.to_bytes()[..], b"hello world"));
    }

}