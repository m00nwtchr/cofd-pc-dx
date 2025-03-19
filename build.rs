use std::process::Command;

fn main() {
	// println!("cargo:rerun-if-changed=./src");
	// println!("cargo:rerun-if-changed=input.css");
	//
	// // Compile TailwindCSS .css file
	// let status = Command::new("tailwindcss")
	// 	.args(["-i", "input.css", "-o", "assets/tailwind.css"])
	// 	.current_dir(std::env!("CARGO_MANIFEST_DIR"))
	// 	.status()
	// 	.expect("TailwindCSS is not installed");
	//
	// if !status.success() {
	// 	panic!("{}", status);
	// }
}
