use librustlet::*; // use the librustlet library
nioruntime_log::debug!(); // set log level to debug

fn main() {
	rustlet_init!(RustletConfig::default()); // initialize with the default config
	rustlet!("myrustlet", { response!("Hello World!"); }); // hello world rustlet
	rustlet_mapping!("/", "myrustlet"); // create mapping to '/'
	std::thread::park(); // park the thread so we don't exit
}
