mod ffi;

use std::env;
use std::process;

fn check_uid_privileges() -> bool {
    unsafe { libc::getuid() == 0 }
}

fn print_usage() {
    println!("Usage: vnv-switcher --target <version_integer_0_to_5>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args != "--target" {
        print_usage();
        process::exit(1);
    }

    let target_version: i32 = match args.parse() {
        Ok(num) if (0..=5).contains(&num) => num,
        _ => {
            eprintln!("[ERR] Invalid target payload boundary. Must be an integer between 0 and 5.");
            process::exit(1);
        }
    };

    if !check_uid_privileges() {
        eprintln!("[ERR] Administrative permissions validation failed. Binary must execute as root inside the target subsystem.");
        process::exit(1);
    }

    println!("[*] Initializing hardware communication abstraction layer...");
    
    match ffi::apply_kernel_version_mask(target_version) {
        Ok(()) => {
            println!("[SUCCESS] Low-level operational hardware vectors modified accurately.");
        }
        Err(ffi::HardwareError::ForkFailure) => {
            eprintln!("[FATAL] Operating system failed to allocate space for system execution fork.");
            process::exit(71);
        }
        Err(ffi::HardwareError::ExecutionInterrupted) => {
            eprintln!("[FATAL] Subsystem execution was forced to terminate prematurely.");
            process::exit(70);
        }
        Err(ffi::HardwareError::UtilityError(exit_code)) => {
            eprintln!("[FATAL] Core system utility rejected payload write commands. Return code: {}", exit_code);
            process::exit(exit_code);
        }
    }
}
