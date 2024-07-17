use jwt_simple::prelude::*;

use crate::User;

const JWT_DURATION: u64 = 60 * 60 * 24;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

#[allow(unused)]
pub struct EncodingKey(Ed25519KeyPair);

#[allow(unused)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    #[allow(unused)]
    pub fn sign(&self, user: impl Into<User>) -> Result<String, jwt_simple::Error> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    #[allow(unused)]
    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let ops = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(ops))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    use super::*;

    #[test]
    fn jwt_encoding_decoding_should_work() -> Result<()> {
        let key = Ed25519KeyPair::generate();
        dbg!(key.public_key().to_pem());
        dbg!(key.to_pem());
        Ok(())
    }

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;
        let user = User::new(1, "ssk", "123@wxample.com");

        let token = ek.sign(user.clone())?;
        let user2 = dk.verify(&token)?;
        assert_eq!(user, user2);
        Ok(())
    }
}
