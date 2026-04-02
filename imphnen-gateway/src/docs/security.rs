use utoipa::{
	Modify,
	openapi::security::{Http, HttpAuthScheme, SecurityRequirement, SecurityScheme},
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}

		let paths = &mut openapi.paths;
		for (_path, path_item) in paths.paths.iter_mut() {
			let process_op = |op: &mut Option<utoipa::openapi::path::Operation>| {
				if let Some(operation) = op.as_mut() {
					let mut has_auth_response = false;
					let responses = &operation.responses.responses;
					for status in responses.keys() {
						if status == "401" || status == "403" {
							has_auth_response = true;
							break;
						}
					}
					if has_auth_response && operation.security.is_none() {
						operation.security = Some(vec![SecurityRequirement::new::<
							&str,
							Vec<&str>,
							&str,
						>("Bearer", vec![])]);
					}
				}
			};

			process_op(&mut path_item.get);
			process_op(&mut path_item.post);
			process_op(&mut path_item.put);
			process_op(&mut path_item.patch);
			process_op(&mut path_item.delete);
			process_op(&mut path_item.options);
			process_op(&mut path_item.head);
			process_op(&mut path_item.trace);
		}
	}
}
