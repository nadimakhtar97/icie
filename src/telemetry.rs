pub struct Counter {
	id: &'static str,
}
impl Counter {
	pub fn spark(&self) {
		evscode::telemetry(self.id, &[], &[])
	}

	pub const fn new(id: &'static str) -> Counter {
		Counter { id }
	}
}

pub struct Events {
	pub auth_ask: Counter,
	pub auth_reset: Counter,
	pub build_all: Counter,
	pub build_manual: Counter,
	pub checker_exists: Counter,
	pub debug_gdb: Counter,
	pub debug_rr: Counter,
	pub discover_start: Counter,
	pub init_countdown: Counter,
	pub init_countdown_ok: Counter,
	pub init_scan: Counter,
	pub init_scan_ok: Counter,
	pub init_url: Counter,
	pub init_url_contest: Counter,
	pub init_url_existing: Counter,
	pub init_url_task: Counter,
	pub launch_nearby: Counter,
	pub launch_web_contest: Counter,
	pub launch_web_task: Counter,
	pub newsletter_show: Counter,
	pub newsletter_changelog: Counter,
	pub newsletter_dismiss: Counter,
	pub paste_qistruct: Counter,
	pub paste_quick: Counter,
	pub paste_quick_ok: Counter,
	pub statement: Counter,
	pub statement_html: Counter,
	pub statement_pdf: Counter,
	pub submit_f12: Counter,
	pub submit_send: Counter,
	pub submit_nolang: Counter,
	pub submit_notask: Counter,
	pub submit_notest: Counter,
	pub submit_failtest: Counter,
	pub template_instantiate: Counter,
	pub template_solution: Counter,
	pub template_solution_custom: Counter,
	pub term_install: Counter,
	pub test_add: Counter,
	pub test_alt0: Counter,
	pub test_alternative_add: Counter,
	pub test_alternative_delete: Counter,
	pub test_current: Counter,
	pub test_edit: Counter,
	pub test_eval: Counter,
	pub test_input: Counter,
	pub test_run: Counter,
	pub v074_migrate_template: Counter,
}

pub static TELEMETRY: Events = Events {
	auth_ask: Counter::new("action.auth_ask"),
	auth_reset: Counter::new("action.auth_reset"),
	build_all: Counter::new("action.build_all"),
	build_manual: Counter::new("action.build_manual"),
	checker_exists: Counter::new("action.checker_exists"),
	debug_gdb: Counter::new("action.debug_gdb"),
	debug_rr: Counter::new("action.debug_rr"),
	discover_start: Counter::new("action.discover_start"),
	init_countdown: Counter::new("action.init_countdown"),
	init_countdown_ok: Counter::new("action.init_countdown_ok"),
	init_scan: Counter::new("action.init_scan"),
	init_scan_ok: Counter::new("action.init_scan_ok"),
	init_url: Counter::new("action.init_url"),
	init_url_contest: Counter::new("action.init_url_contest"),
	init_url_existing: Counter::new("action.init_url_existing"),
	init_url_task: Counter::new("action.init_url_task"),
	launch_nearby: Counter::new("action.launch_nearby"),
	launch_web_contest: Counter::new("action.launch_web_contest"),
	launch_web_task: Counter::new("action.launch_web_task"),
	newsletter_show: Counter::new("action.newsletter_show"),
	newsletter_changelog: Counter::new("action.newsletter_changelog"),
	newsletter_dismiss: Counter::new("action.newsletter_dismiss"),
	paste_qistruct: Counter::new("action.paste_qistruct"),
	paste_quick: Counter::new("action.paste_quick"),
	paste_quick_ok: Counter::new("action.paste_quick_ok"),
	statement: Counter::new("action.statement"),
	statement_html: Counter::new("action.statement_html"),
	statement_pdf: Counter::new("action.statement_pdf"),
	submit_f12: Counter::new("action.submit_f12"),
	submit_send: Counter::new("action.submit_send"),
	submit_nolang: Counter::new("action.submit_nolang"),
	submit_notask: Counter::new("action.submit_notask"),
	submit_notest: Counter::new("action.submit_notests"),
	submit_failtest: Counter::new("action.submit_failtest"),
	template_instantiate: Counter::new("action.template_instantiate"),
	template_solution: Counter::new("action.template_solution"),
	template_solution_custom: Counter::new("action.template_solution_custom"),
	term_install: Counter::new("action.term_install"),
	test_add: Counter::new("action.test_add"),
	test_alt0: Counter::new("action.test_alt0"),
	test_alternative_add: Counter::new("action.test_alternative_add"),
	test_alternative_delete: Counter::new("action.test_alternative_delete"),
	test_current: Counter::new("action.test_current"),
	test_edit: Counter::new("action.test_edit"),
	test_eval: Counter::new("action.test_eval"),
	test_input: Counter::new("action.test_input"),
	test_run: Counter::new("action.test_run"),
	v074_migrate_template: Counter::new("migrate.v074_template"),
};
