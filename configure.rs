#[link(name = "configure.rs",
       description = "Configuration utility for Tourney",
       author = "Arcterus",
       vers = "0.1",
       license = "MPL v2.0")];

extern mod extra;
use extra::getopts::*;
use std::os;
use std::io;
use std::iterator::IteratorUtil;

fn main() {
	let args = os::args();

	let program = copy args[0];

	let opts = ~[
		optopt("target"),
		optopt("with-cc"),
		optopt("with-cxx"),
		optopt("with-android-sdk"),
		optflag("h"),
		optflag("help")
	];

	let matches = match getopts(args.tail(), opts) {
		Ok(m) => m,
		Err(f) => fail!(fail_str(f))
	};

	if opt_present(&matches, "h") || opt_present(&matches, "help") {
		print_help(program);
	} else {
		let target = match opt_maybe_str(&matches, "target") {
			Some(tgt) => tgt,
			None => fmt!("%s", os::SYSNAME)
		};
		configure(target, opt_maybe_str(&matches, "with-cc"),
					 opt_maybe_str(&matches, "with-cxx"),
					 opt_maybe_str(&matches, "with-android-sdk"));
		println("Configuration successful!");
	}
}

fn print_help(prog: &str) {
	println(fmt!("Usage: %s [options]", prog));
	println("Options:");
	println("\t--target [tgt]\t\tSet tgt to an available target 
(i.e. macos, linux)");
	println("\t--with-cc [cc]\t\tSet cc to the given path");
	println("\t--with-cxx [cxx]\t\tSet cxx to the given path");
	println("\t--with-android-sdk [sdk]\t\tSet the path to the Android SDK");
	println("\t-h | --help\t\tPrint out this help menu");
}

fn configure(target: &str, cc: Option<~str>, cxx: Option<~str>,
				 andysdk: Option<~str>) {
	let config = match io::mk_file_writer(&Path("config.ninja"),
													  [io::Create, io::Truncate]) {
		Ok(writer) => writer,
		Err(f) => fail!(f)
	};
	match target {
		"android" => {
			check_for_sdk(copy andysdk);
			config.write_line(fmt!("andysdk = %s", andysdk.unwrap()));
			config.write_line("target = arm-linux-androideabi");
		}
		_ => {
			config.write_line(fmt!("cc = %s", match cc {
				Some(comp) => { check_for_binary(comp); comp }
				None => { check_for_binary("clang"); ~"clang" }
			}));
			config.write_line(fmt!("cxx = %s", match cxx {
				Some(comp) => { check_for_binary(comp); comp }
				None => { check_for_binary("clang++");	~"clang++" }
			}));
			check_for_gfxlib("Qt4");
			config.write_line("gfxlib = Qt4");
			config.write_line(fmt!("target = %s-%s", os::ARCH, match target {
												"linux" => "unknown-linux-gnu",
												"macos" => "apple-darwin",
												"freebsd" => "unknown-freebsd",
												"win32" => "pc-mingw32",
												_ => fail!("Error: unknown target (%s)", target)
										  }));
		}
	}
	os::copy_file(&Path(fmt!("config/%s.ninja", target)), &Path("target.ninja"));
}

fn check_for_sdk(sdk: Option<~str>) {
	match sdk {
		Some(dir) => if !os::path_is_dir(&Path(dir)) {
			fail!(fmt!("Error: %s does not point to a possible Android SDK", dir));
		},
		None => { fail!("Error: no Android SDK given"); }
	}
}

fn check_for_binary(prog: &str) {
	if !check_for_file(prog, binary_path()) {
		fail!(fmt!("Error: could not find program %s", prog));
	}
}

fn check_for_lib(libname: &str) {
	let lib = fmt!("%s%s%s", os::DLL_PREFIX, libname, os::DLL_SUFFIX);
	if !check_for_file(lib, library_path()) {
		println(lib);
		fail!(fmt!("Error: could not find library %s", libname));
	}
}

fn check_for_file(file: &str, path: &[&str]) -> bool {
	for path.each() |dir| {
		if os::path_exists(&Path(fmt!("%s/%s", *dir, file))) {
			return true;
		}
	}
	false
}

#[cfg(unix)]
fn library_path() -> ~[&str] {
	~["/usr/lib", "/usr/lib/local", "/lib"]
}

#[cfg(unix)]
fn binary_path() -> ~[&str] {
	match os::getenv("PATH") {
		Some(path) => {
			let cont: ~[&str] = path.split_iter(':').collect();
			cont
		}
		None => ~["/usr/local/bin", "/usr/bin", "/bin", "/usr/local/sbin", "/usr/sbin", "/sbin"] // defaults
	}
}

#[cfg(windows)]
fn library_path() -> ~[&str] {
	~[] // not sure what to do here
}

#[cfg(windows)]
fn library_path() -> ~[&str] {
	~[] // mostly same as unix, but with different defaults
}

fn check_for_gfxlib(lib: &str) {
	match lib {
		"Qt4" => check_for_lib("QtGui"),
		_ => fail!(fmt!("Error: unknown graphics library (%s)", lib))
	};
}
