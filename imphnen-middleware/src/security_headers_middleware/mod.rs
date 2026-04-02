use axum::{
	Extension,
	http::{HeaderValue, Request, Response},
	middleware::Next,
};
use imphnen_libs::{AppState, ENV};
use rand::RngCore;
use std::convert::Infallible;

pub async fn security_headers_middleware(
	Extension(_state): Extension<AppState>,
	req: Request<axum::body::Body>,
	next: Next,
) -> Result<Response<axum::body::Body>, Infallible> {
	let nonce = if ENV.rust_env != "production" {
		generate_nonce()
	} else {
		String::new()
	};

	let res = next.run(req).await;

	let res = add_security_headers(res, &nonce);

	Ok(res)
}

fn add_security_headers(
	mut res: Response<axum::body::Body>,
	nonce: &str,
) -> Response<axum::body::Body> {
	let headers = res.headers_mut();

	if ENV.rust_env == "production" {
		headers.insert(
			"Strict-Transport-Security",
			HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
		);
	} else {
		headers.insert(
			"Strict-Transport-Security",
			HeaderValue::from_static("max-age=0"),
		);
	}

	let csp = if ENV.rust_env == "production" {
		"default-src 'self'; script-src 'self' https://trusted-cdn.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' data: https://images.example.com; connect-src 'self' https://api.example.com; frame-src 'none'; object-src 'none'; base-uri 'self'; form-action 'self'; report-uri /csp-violation-report-endpoint".to_string()
	} else {
		if nonce.is_empty() {
			"default-src 'self' http://localhost:3000; script-src 'self' http://localhost:3000; style-src 'self' http://localhost:3000; img-src 'self' data: http://localhost:3000; connect-src 'self' http://localhost:3000 ws://localhost:3000; frame-src 'none'; object-src 'none'; base-uri 'self'; form-action 'self'".to_string()
		} else {
			format!(
				"default-src 'self' http://localhost:3000; script-src 'self' http://localhost:3000 'nonce-{}'; style-src 'self' http://localhost:3000 'nonce-{}'; img-src 'self' data: http://localhost:3000; connect-src 'self' http://localhost:3000 ws://localhost:3000; frame-src 'none'; object-src 'none'; base-uri 'self'; form-action 'self'",
				nonce, nonce
			)
		}
	};

	if let Ok(csp_value) = HeaderValue::from_str(&csp) {
		headers.insert("Content-Security-Policy", csp_value);
	}

	if ENV.rust_env != "production"
		&& !nonce.is_empty()
		&& let Ok(nonce_value) = HeaderValue::from_str(nonce)
	{
		headers.insert("X-CSP-Nonce", nonce_value);
	}

	headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));

	headers.insert(
		"X-Content-Type-Options",
		HeaderValue::from_static("nosniff"),
	);

	headers.insert(
		"Referrer-Policy",
		HeaderValue::from_static("strict-origin-when-cross-origin"),
	);

	headers.insert(
		"Permissions-Policy",
		HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
	);

	headers.insert(
		"X-XSS-Protection",
		HeaderValue::from_static("1; mode=block"),
	);

	res
}

fn generate_nonce() -> String {
	use base64::{Engine as _, engine::general_purpose::STANDARD};
	let mut rng = rand::rng();
	let mut random_bytes = [0u8; 16];
	rng.fill_bytes(&mut random_bytes);
	STANDARD.encode(random_bytes)
}
