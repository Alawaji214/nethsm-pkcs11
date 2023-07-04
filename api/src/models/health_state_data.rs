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
pub struct HealthStateData {
    #[serde(rename = "state")]
    pub state: crate::models::SystemState,
}

impl HealthStateData {
    pub fn new(state: crate::models::SystemState) -> HealthStateData {
        HealthStateData { state }
    }
}
