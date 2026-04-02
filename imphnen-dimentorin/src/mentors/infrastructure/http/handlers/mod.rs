pub mod mutation_handlers;
pub mod query_handlers;

pub use mutation_handlers::{
	delete_mentor, post_register_mentor, put_update_mentor, put_update_mentor_me,
	put_update_mentor_no_id, put_verify_mentor,
};
pub use query_handlers::{
	get_mentor_by_id, get_mentor_list, get_mentor_me, get_mentor_status,
};
