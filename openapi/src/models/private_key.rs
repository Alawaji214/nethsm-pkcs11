/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All [base64](https://tools.ietf.org/html/rfc4648#section-4) encoded values are Big Endian. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateKey {
    #[serde(rename = "mechanisms")]
    pub mechanisms: Vec<crate::models::KeyMechanism>,
    #[serde(rename = "type")]
    pub r#type: crate::models::KeyType,
    #[serde(rename = "key")]
    pub key: Box<crate::models::KeyPrivateData>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<crate::models::KeyRestrictions>>,
}

impl PrivateKey {
    pub fn new(mechanisms: Vec<crate::models::KeyMechanism>, r#type: crate::models::KeyType, key: crate::models::KeyPrivateData) -> PrivateKey {
        PrivateKey {
            mechanisms,
            r#type,
            key: Box::new(key),
            restrictions: None,
        }
    }
}

