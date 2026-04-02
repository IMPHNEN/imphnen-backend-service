use anyhow::Result;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

#[allow(clippy::too_many_arguments)]
pub fn compute_header_auth(
	method: &str,
	host: &str,
	canonical_uri: &str,
	canonical_query: &str,
	payload_hash: &str,
	access_key: &str,
	secret_key: &str,
	region: &str,
) -> Result<(String, String)> {
	let now = Utc::now();
	let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
	let date_stamp = now.format("%Y%m%d").to_string();
	let scope = format!("{date_stamp}/{region}/s3/aws4_request");

	let canonical_headers = format!(
		"host:{host}\nx-amz-content-sha256:{payload_hash}\nx-amz-date:{amz_date}\n"
	);
	let signed_headers = "host;x-amz-content-sha256;x-amz-date";
	let canonical_request = format!(
		"{method}\n{canonical_uri}\n{canonical_query}\n{canonical_headers}\n{signed_headers}\n{payload_hash}"
	);

	let string_to_sign = format!(
		"AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n{}",
		hex::encode(Sha256::digest(canonical_request.as_bytes()))
	);

	let signing_key = derive_signing_key(secret_key, &date_stamp, region)?;
	let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key)?;
	mac.update(string_to_sign.as_bytes());
	let signature = hex::encode(mac.finalize().into_bytes());

	let auth_header = format!(
		"AWS4-HMAC-SHA256 Credential={access_key}/{scope}, SignedHeaders={signed_headers}, Signature={signature}"
	);
	Ok((auth_header, amz_date))
}

pub fn compute_presigned_url(
	host: &str,
	bucket: &str,
	object_name: &str,
	expiry_seconds: u32,
	access_key: &str,
	secret_key: &str,
	region: &str,
) -> Result<String> {
	let now = Utc::now();
	let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
	let date_stamp = now.format("%Y%m%d").to_string();
	let scope = format!("{date_stamp}/{region}/s3/aws4_request");
	let credential = format!("{access_key}/{scope}");
	let expires_str = expiry_seconds.to_string();

	let mut query_params = std::collections::BTreeMap::new();
	query_params.insert("X-Amz-Algorithm", "AWS4-HMAC-SHA256");
	query_params.insert("X-Amz-Credential", &credential);
	query_params.insert("X-Amz-Date", &amz_date);
	query_params.insert("X-Amz-Expires", &expires_str);
	query_params.insert("X-Amz-SignedHeaders", "host");

	let canonical_query_string = query_params
		.iter()
		.map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
		.collect::<Vec<_>>()
		.join("&");

	let canonical_request = format!(
		"GET\n/{bucket}/{object_name}\n{canonical_query_string}\nhost:{host}\n\nhost\nUNSIGNED-PAYLOAD"
	);

	let string_to_sign = format!(
		"AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n{}",
		hex::encode(Sha256::digest(canonical_request.as_bytes()))
	);

	let signing_key = derive_signing_key(secret_key, &date_stamp, region)?;
	let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key)?;
	mac.update(string_to_sign.as_bytes());
	let signature = hex::encode(mac.finalize().into_bytes());

	Ok(format!(
		"https://{host}/{bucket}/{object_name}?{canonical_query_string}&X-Amz-Signature={signature}"
	))
}

pub(crate) fn derive_signing_key(
	secret_key: &str,
	date_stamp: &str,
	region: &str,
) -> Result<Vec<u8>> {
	let secret = format!("AWS4{secret_key}");
	let mut mac1 = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
	mac1.update(date_stamp.as_bytes());
	let date_key = mac1.finalize().into_bytes();

	let mut mac2 = Hmac::<Sha256>::new_from_slice(&date_key)?;
	mac2.update(region.as_bytes());
	let date_region_key = mac2.finalize().into_bytes();

	let mut mac3 = Hmac::<Sha256>::new_from_slice(&date_region_key)?;
	mac3.update(b"s3");
	let date_region_service_key = mac3.finalize().into_bytes();

	let mut mac4 = Hmac::<Sha256>::new_from_slice(&date_region_service_key)?;
	mac4.update(b"aws4_request");
	Ok(mac4.finalize().into_bytes().to_vec())
}
