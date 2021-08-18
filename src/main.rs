use librustlet::*;
log::debug!();

fn main() {
	rustlet_init!(RustletConfig::default());
	rustlet!("myrustlet", { response!("Hello World!"); });
	rustlet_mapping!("/myrustlet", "myrustlet");
	std::thread::park();
}
