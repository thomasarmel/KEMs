use crate::DhKem;
use kem::{Decapsulate, Encapsulate};
use rand::thread_rng;

trait SecretBytes {
    fn as_slice(&self) -> &[u8];
}

#[cfg(feature = "x25519")]
impl SecretBytes for x25519::SharedSecret {
    fn as_slice(&self) -> &[u8] {
        self.as_bytes().as_slice()
    }
}

#[cfg(feature = "arithmetic")]
impl<C> SecretBytes for elliptic_curve::ecdh::SharedSecret<C>
where
    C: elliptic_curve::CurveArithmetic,
{
    fn as_slice(&self) -> &[u8] {
        self.raw_secret_bytes().as_slice()
    }
}

// we need this because if the crate is compiled with no features this function never
// gets used
#[allow(dead_code)]
fn test_kem<K: DhKem>()
where
    <K as DhKem>::SharedSecret: SecretBytes,
{
    let mut rng = thread_rng();
    let (sk, pk) = K::random_keypair(&mut rng);
    let (ek, ss1) = pk.encapsulate(&mut rng).expect("never fails");
    let ss2 = sk.decapsulate(&ek).expect("never fails");

    assert_eq!(ss1.as_slice(), ss2.as_slice());
}

#[cfg(feature = "x25519")]
#[test]
fn test_x25519() {
    test_kem::<crate::X25519>();
}

#[cfg(feature = "bign256")]
#[test]
fn test_bign256() {
    test_kem::<crate::BignP256>();
}

#[cfg(feature = "k256")]
#[test]
fn test_k256() {
    test_kem::<crate::Secp256k1>();
}

#[cfg(feature = "p192")]
#[test]
fn test_p192() {
    test_kem::<crate::NistP192>();
}

#[cfg(feature = "p224")]
#[test]
fn test_p224() {
    test_kem::<crate::NistP224>();
}

#[cfg(feature = "p256")]
#[test]
fn test_p256() {
    test_kem::<crate::NistP256>();
}

#[cfg(feature = "p384")]
#[test]
fn test_p384() {
    test_kem::<crate::NistP384>();
}

#[cfg(feature = "p521")]
#[test]
fn test_p521() {
    test_kem::<crate::NistP521>();
}

#[cfg(feature = "sm2")]
#[test]
fn test_sm2() {
    test_kem::<crate::Sm2>();
}
