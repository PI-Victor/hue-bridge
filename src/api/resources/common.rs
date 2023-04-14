use super::ResourceTypes;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ResourceIdentifier {
    rid: String,
    r#type: ResourceTypes,
}
