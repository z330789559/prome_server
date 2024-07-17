

//f2 d6 a3 be ae 4a 88 d5 ff 22 a0 e0 0c 6c ae be 3b 0b b7 03 f6 a9 db 32 6b 20 d3 9a 36 a8 81 f2
const PRIVATE_KEY: [u8;32] =[
   0xf2, 0xd6, 0xa3, 0xbe, 0xae, 0x4a, 0x88, 0xd5, 0xff, 0x22, 0xa0, 0xe0, 0x0c, 0x6c, 0xae, 0xbe,
    0x3b, 0x0b, 0xb7, 0x03, 0xf6, 0xa9, 0xdb, 0x32, 0x6b, 0x20, 0xd3, 0x9a, 0x36, 0xa8, 0x81, 0xf2
];
//d8 48 bb bb f9 ad ae 64 bd 13 bc 67 3b fb 10 d0 d5 b8 cb 0c 65 ac af dc bd 90 4b 83 04 82 bd 76
const PUBLIC_KEY: [u8;32]=[
    0xd8, 0x48, 0xbb, 0xbb, 0xf9, 0xad, 0xae, 0x64, 0xbd, 0x13, 0xbc, 0x67, 0x3b, 0xfb, 0x10, 0xd0,
    0xd5, 0xb8, 0xcb, 0x0c, 0x65, 0xac, 0xaf, 0xdc, 0xbd, 0x90, 0x4b, 0x83, 0x04, 0x82, 0xbd, 0x76
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

        //90 e4 fb 09 0c ad 59 ad c8 31 87 ad 30 0b 3d 2c bd 86 d2 cc e0 40 e5 e5 67 bf 2f cb 32 f0 d3 4b 38 0e eb be 80 1f 2d fa 70 fb 34 44 82 54 1b 52 ce ce 48 05 3e f6 0d 3d 76 73 33 e4 1e c0 bd 08
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

         // let mut sk = Keypair::from_bytes(&buffer).unwrap();
        // let pk = sk.verifying_key();
        // let vk =VerifyingKey::from_bytes(&public_key).unwrap();
        //68,65,6c,6c,6f,21,21,21
        let message =[0x68,0x65,0x6c,0x6c,0x6f,0x21,0x21,0x21];
        const sign_data: [u8; 64] =[0x90,0xe4,0xfb,0x09,0x0c,0xad,0x59,0xad,0xc8,0x31,0x87,0xad,0x30,0x0b,0x3d,0x2c,0xbd,0x86,0xd2,0xcc,0xe0,0x40,0xe5,0xe5,0x67,0xbf,0x2f,0xcb,0x32,0xf0,0xd3,0x4b,0x38,0x0e,0xeb,0xbe,0x80,0x1f,0x2d,0xfa,0x70,0xfb,0x34,0x44,0x82,0x54,0x1b,0x52,0xce,0xce,0x48,0x05,0x3e,0xf6,0x0d,0x3d,0x76,0x73,0x33,0xe4,0x1e,0xc0,0xbd,0x08];
       let signatures= Signature::from(sign_data);

        let mut sk = Keypair::new();

        let signature1=    sk.sign_message(b"hello world");
        println!("signature1: {:?}", signature1.to_string());
        println!("signatures: {:?}", signatures.to_string());
        println!("address {:?}", sk.pubkey().to_string());
        assert!(signature1.verify(&sk.pubkey().to_bytes()[..], &message), "not ok");
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
        const sign_data: [i32; 64] =[0x90,0xe4,0xfb,0x09,0x0c,0xad,0x59,0xad,0xc8,0x31,0x87,0xad,0x30,0x0b,0x3d,0x2c,0xbd,0x86,0xd2,0xcc,0xe0,0x40,0xe5,0xe5,0x67,0xbf,0x2f,0xcb,0x32,0xf0,0xd3,0x4b,0x38,0x0e,0xeb,0xbe,0x80,0x1f,0x2d,0xfa,0x70,0xfb,0x34,0x44,0x82,0x54,0x1b,0x52,0xce,0xce,0x48,0x05,0x3e,0xf6,0x0d,0x3d,0x76,0x73,0x33,0xe4,0x1e,0xc0,0xbd,0x08];

       // let sig=Signature::from_str(signature).unwrap();
       //  assert!(sig.verify(&pk.to_bytes()[..], b"hello world"));
       //  assert!(sig.verify(&pk.to_bytes()[..], b"hello world"));
    }

}