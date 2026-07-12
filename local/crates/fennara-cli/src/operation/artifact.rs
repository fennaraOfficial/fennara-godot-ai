use serde_json::{Map, Value, json};

pub(super) fn select(
    artifacts: &mut Map<String, Value>,
    name: String,
    expected_sha256: Option<String>,
) -> Result<(), String> {
    let artifact = artifacts.entry(name.clone()).or_insert_with(|| json!({}));
    let object = artifact
        .as_object_mut()
        .ok_or_else(|| format!("operation artifact {name} has an invalid state"))?;
    object.insert("asset".into(), Value::String(name));
    if expected_sha256.is_some() || !object.contains_key("expected_sha256") {
        object.insert(
            "expected_sha256".into(),
            expected_sha256.map(Value::String).unwrap_or(Value::Null),
        );
    }
    object.insert("status".into(), Value::String("selected".into()));
    Ok(())
}

pub(super) fn record_hash(
    artifacts: &mut Map<String, Value>,
    name: String,
    actual_sha256: String,
    verified: Option<bool>,
) -> Result<(), String> {
    let artifact = artifacts
        .entry(name.clone())
        .or_insert_with(|| json!({ "asset": name }));
    let object = artifact
        .as_object_mut()
        .ok_or_else(|| "operation artifact has an invalid state".to_string())?;
    object.insert("actual_sha256".into(), Value::String(actual_sha256));
    object.insert(
        "status".into(),
        Value::String(
            match verified {
                Some(true) => "verified",
                Some(false) => "mismatch",
                None => "downloaded",
            }
            .into(),
        ),
    );
    Ok(())
}
