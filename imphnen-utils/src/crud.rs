pub trait Crud<T, Args> {
	fn list(&self, _args: Args) -> T {
		unimplemented!("list() is not implemented for this type")
	}
	fn detail(&self, _args: Args) -> T {
		unimplemented!("detail() is not implemented for this type")
	}
}
