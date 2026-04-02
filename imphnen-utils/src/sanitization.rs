use regex::Regex;
use std::sync::LazyLock;

static SQL_INJECTION_PATTERNS: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"(?i)(union|select|insert|update|delete|drop|create|alter|truncate|vacuum|analyze|reindex|cluster|copy|exec|script|javascript|onerror|onload|with|from|where|join|group by|order by|limit|offset|having|distinct|into|values|union all|union distinct|::|%|:=|current_user|session_user|user|version|current_date|current_time|now|pg_sleep|pg_user|pg_database|pg_tables|pg_columns|chr|ascii|substring|position|strpos|concat|concat_ws|string_agg|array_agg|array_to_string|string_to_array)").expect("valid sql injection regex")
});

static PATH_TRAVERSAL_REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"\.\.(/|\\)").expect("valid path traversal regex"));

pub fn sanitize_html(input: &str) -> String {
	input
		.chars()
		.map(|c| match c {
			'<' => "&lt;".to_string(),
			'>' => "&gt;".to_string(),
			'"' => "&quot;".to_string(),
			'\'' => "&#39;".to_string(),
			'&' => "&amp;".to_string(),
			_ => c.to_string(),
		})
		.collect()
}

pub fn sanitize_dangerous_patterns(input: &str) -> String {
	let without_sql_injection =
		SQL_INJECTION_PATTERNS.replace_all(input, "[FILTERED]");
	let without_postgres_specific =
		without_sql_injection.replace(";--", ";[FILTERED]");
	without_postgres_specific.to_owned()
}

pub fn contains_path_traversal(input: &str) -> bool {
	PATH_TRAVERSAL_REGEX.is_match(input)
}

pub fn sanitize_filename(input: &str) -> String {
	input
		.chars()
		.map(|c| match c {
			'/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
			c if c.is_control() => '_',
			c => c,
		})
		.collect()
}

pub fn sanitize_user_text(input: &str) -> String {
	let without_html = sanitize_html(input);
	sanitize_dangerous_patterns(&without_html)
}

pub fn normalize_whitespace(input: &str) -> String {
	input
		.split_whitespace()
		.collect::<Vec<_>>()
		.join(" ")
		.trim()
		.to_string()
}

pub fn sanitize_email(email: &str) -> Option<String> {
	let trimmed = email.trim().to_lowercase();

	if trimmed.contains('@') && trimmed.contains('.') {
		Some(trimmed)
	} else {
		None
	}
}

pub fn sanitize_url(url: &str) -> Option<String> {
	let trimmed = url.trim();

	let lower = trimmed.to_lowercase();
	if lower.starts_with("javascript:")
		|| lower.starts_with("data:")
		|| lower.starts_with("vbscript:")
	{
		return None;
	}

	if lower.starts_with("http://")
		|| lower.starts_with("https://")
		|| lower.starts_with("/")
	{
		Some(trimmed.to_string())
	} else {
		None
	}
}
