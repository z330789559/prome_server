use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use hex;
use tiny_keccak::{Hasher, Keccak};
use ecdsa::Signature;

fn fn_verify(str_src: &str, str_sign: &str, public_key: &str) -> bool {
    // 将源字符串转换为字节数组
    let byte_str: &[u8] = str_src.as_bytes();

    // 生成 Secp256k1 上下文
    let secp = Secp256k1::new();

    // 从十六进制字符串解析公钥
    let public_key_bytes = hex::decode(public_key).expect("Invalid public key hex format");
    let public_key = PublicKey::from_slice(&public_key_bytes).expect("Invalid public key");

    // 解析签名
    let signature_bytes = hex::decode(str_sign).expect("Invalid signature hex format");
    let signature = Signature::from_asn1(&signature_bytes).expect("Invalid signature format");

    // 生成消息摘要
    let message = Message::from_slice(&byte_str).expect("Invalid message length");

    // 验证签名
    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}

fn fn_sign(str_src: &str, private_key: &[u8]) -> String {
    // 创建 Secp256k1 上下文
    let secp = Secp256k1::new();

    // 从私钥字节创建 SecretKey
    let secret_key = SecretKey::from_slice(private_key).expect("Invalid private key");

    // 创建消息对象
    let message = Message::from_slice(&keccak256(str_src.as_bytes())).expect("Invalid message length");

    // 签名消息
    let (signature) = secp.sign_ecdsa(&message, &secret_key);

    // 返回签名的十六进制表示
    hex::encode(signature.serialize_compact())
}

// 计算 Keccak256 哈希
fn keccak256(input: &[u8]) -> Vec<u8> {

    let mut hasher = Keccak::v256(); // Keccak-256
    let mut result = [0u8; 32]; // 256 bits / 8 = 32 bytes
    hasher.update(input);
    hasher.finalize(&mut result);
    result.to_vec()
}