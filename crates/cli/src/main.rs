use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arch {
    Arm64,
    X86_64,
    Armv7a,
}

impl Arch {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "arm64" | "aarch64" => Ok(Self::Arm64),
            "x86_64" => Ok(Self::X86_64),
            "armv7a" | "armv7" => Ok(Self::Armv7a),
            _ => Err(format!(
                "unsupported architecture {value:?}; expected arm64, x86_64, or armv7a"
            )),
        }
    }

    fn rust_target(self) -> &'static str {
        match self {
            Self::Arm64 => "aarch64-unknown-linux-ohos",
            Self::X86_64 => "x86_64-unknown-linux-ohos",
            Self::Armv7a => "armv7-unknown-linux-ohos",
        }
    }

    fn clang_triple(self) -> &'static str {
        match self {
            Self::Arm64 => "aarch64-unknown-linux-ohos",
            Self::X86_64 => "x86_64-unknown-linux-ohos",
            Self::Armv7a => "armv7-unknown-linux-ohos",
        }
    }
}

struct Options {
    arch: Arch,
    sdk_root: PathBuf,
    release: bool,
    cargo_args: Vec<String>,
    module_descriptor: Option<String>,
    ets_output: Option<PathBuf>,
    ets_library: Option<String>,
}

fn default_sdk_root() -> PathBuf {
    env::var_os("DEVECO_SDK_ROOT").map_or_else(
        || PathBuf::from("/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony"),
        PathBuf::from,
    )
}

fn parse_options(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut arch = Arch::Arm64;
    let mut sdk_root = default_sdk_root();
    let mut release = false;
    let mut cargo_args = Vec::new();
    let mut module_descriptor = None;
    let mut ets_output = None;
    let mut ets_library = None;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--arch" => {
                arch = Arch::parse(
                    &args
                        .next()
                        .ok_or_else(|| "--arch requires a value".to_string())?,
                )?;
            }
            "--sdk" => {
                sdk_root = PathBuf::from(
                    args.next()
                        .ok_or_else(|| "--sdk requires a path".to_string())?,
                );
            }
            "--release" => release = true,
            "--module-descriptor" => {
                module_descriptor = Some(
                    args.next()
                        .ok_or_else(|| "--module-descriptor requires a value".to_string())?,
                );
            }
            "--ets-output" => {
                ets_output = Some(PathBuf::from(
                    args.next()
                        .ok_or_else(|| "--ets-output requires a path".to_string())?,
                ));
            }
            "--library" => {
                ets_library = Some(
                    args.next()
                        .ok_or_else(|| "--library requires a value".to_string())?,
                );
            }
            "--" => {
                cargo_args.extend(args);
                break;
            }
            "-h" | "--help" => return Err(usage()),
            other => cargo_args.push(other.to_string()),
        }
    }
    Ok(Options {
        arch,
        sdk_root,
        release,
        cargo_args,
        module_descriptor,
        ets_output,
        ets_library,
    })
}

fn tool_path(options: &Options, suffix: &str) -> PathBuf {
    options
        .sdk_root
        .join("native/llvm/bin")
        .join(format!("{}-{suffix}", options.arch.clang_triple()))
}

fn validate(options: &Options) -> Result<(), String> {
    let clang = tool_path(options, "clang");
    let clangxx = tool_path(options, "clang++");
    for required in [&clang, &clangxx, &options.sdk_root.join("native/sysroot")] {
        if !required.exists() {
            return Err(format!(
                "missing OpenHarmony SDK path: {}",
                required.display()
            ));
        }
    }

    let rustup = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map_err(|error| format!("failed to execute rustup: {error}"))?;
    if !rustup.status.success() {
        return Err("rustup target list --installed failed".to_string());
    }
    let installed = String::from_utf8_lossy(&rustup.stdout);
    if !installed
        .lines()
        .any(|line| line == options.arch.rust_target())
    {
        return Err(format!(
            "Rust target {} is missing; run `rustup target add {}`",
            options.arch.rust_target(),
            options.arch.rust_target()
        ));
    }
    Ok(())
}

fn doctor(options: &Options) -> Result<(), String> {
    validate(options)?;
    println!("SDK: {}", options.sdk_root.display());
    println!("architecture: {:?}", options.arch);
    println!("Rust target: {}", options.arch.rust_target());
    println!("C compiler: {}", tool_path(options, "clang").display());
    if let Some(source) = env::var_os("OHOS_SOURCE_ROOT") {
        let es2panda =
            Path::new(&source).join("out/arm64_virt/clang_x64/arkcompiler/ets_frontend/es2panda");
        println!(
            "es2panda: {} ({})",
            es2panda.display(),
            if es2panda.is_file() {
                "available"
            } else {
                "missing"
            }
        );
    }
    println!("ani-rs doctor: OK");
    Ok(())
}

fn build(options: &Options) -> Result<(), String> {
    validate(options)?;
    let target = options.arch.rust_target();
    let clang = tool_path(options, "clang");
    let clangxx = tool_path(options, "clang++");
    let cargo_linker = format!(
        "CARGO_TARGET_{}_LINKER",
        target.replace('-', "_").to_ascii_uppercase()
    );
    let cc = format!("CC_{}", target.replace('-', "_"));
    let cxx = format!("CXX_{}", target.replace('-', "_"));

    let mut command = Command::new("cargo");
    command.args(["build", "--target", target]);
    if options.release {
        command.arg("--release");
    }
    command.args(&options.cargo_args);
    command
        .env(cargo_linker, &clang)
        .env(cc, &clang)
        .env(cxx, &clangxx);
    if let Some(descriptor) = &options.module_descriptor {
        command.env("ANI_MODULE_DESCRIPTOR", descriptor);
    }
    if let Some(output) = &options.ets_output {
        command.env("ANI_ETS_OUTPUT", output);
    }
    if let Some(library) = &options.ets_library {
        command.env("ANI_ETS_LIBRARY", library);
    }
    let status = command
        .status()
        .map_err(|error| format!("failed to execute cargo: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo build failed with {status}"))
    }
}

fn run_repo_script(name: &str, args: &[String], options: &Options) -> Result<(), String> {
    let script = env::current_dir()
        .map_err(|error| format!("failed to resolve current directory: {error}"))?
        .join("scripts")
        .join(name);
    if !script.is_file() {
        return Err(format!(
            "{} must be run from an ani-rs checkout (missing {})",
            name,
            script.display()
        ));
    }
    let guest_arch = match options.arch {
        Arch::Arm64 => "arm64",
        Arch::X86_64 => "x86_64",
        Arch::Armv7a => "armv7a",
    };
    let mut command = Command::new(script);
    command
        .args(args)
        .env("DEVECO_SDK_ROOT", &options.sdk_root)
        .env("OHOS_QEMU_GUEST_ARCH", guest_arch);
    let status = command
        .status()
        .map_err(|error| format!("failed to run {name}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{name} failed with {status}"))
    }
}

fn usage() -> String {
    "\
ani-rs - OpenHarmony ANI build helper

Usage:
  ani-rs doctor [--arch arm64|x86_64|armv7a] [--sdk PATH]
  ani-rs build  [--arch ARCH] [--sdk PATH] [--release]
                [--module-descriptor NAME] [--ets-output PATH] [--library NAME]
                [-- CARGO_ARGS...]
  ani-rs hap    [--arch ARCH] [--sdk PATH]
  ani-rs hap-repro [--arch ARCH] [--sdk PATH]
  ani-rs qemu   [--arch ARCH] [--sdk PATH]
  ani-rs qemu-memory [--arch ARCH] [--sdk PATH]
  ani-rs hap-qemu [--arch ARCH] [--sdk PATH] [-- HAP_PATH]
  ani-rs verify-hap [--arch ARCH] [--sdk PATH] -- HAP_PATH
"
    .to_string()
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    let options = parse_options(args)?;
    match command.as_str() {
        "doctor" => doctor(&options),
        "build" => build(&options),
        "hap" => run_repo_script(
            "build_hap_smoke.sh",
            &[match options.arch {
                Arch::Arm64 => "arm64",
                Arch::X86_64 => "x86_64",
                Arch::Armv7a => "armv7a",
            }
            .to_string()],
            &options,
        ),
        "hap-repro" => run_repo_script(
            "check_hap_reproducible.sh",
            &[match options.arch {
                Arch::Arm64 => "arm64",
                Arch::X86_64 => "x86_64",
                Arch::Armv7a => "armv7a",
            }
            .to_string()],
            &options,
        ),
        "qemu" => {
            // The runtime script intentionally consumes its configuration via
            // environment variables so CI and local runs share one path.
            run_repo_script("run_arkvm_examples_ohos_qemu.sh", &[], &options)
        }
        "qemu-memory" => run_repo_script("check_qemu_memory.sh", &[], &options),
        "hap-qemu" => {
            let arch = match options.arch {
                Arch::Arm64 => "arm64",
                Arch::X86_64 => "x86_64",
                Arch::Armv7a => "armv7a",
            };
            let mut script_args = options.cargo_args.clone();
            if script_args.is_empty() {
                script_args.push(String::new());
            }
            script_args.push(arch.to_string());
            run_repo_script("run_hap_abc_ohos_qemu.sh", &script_args, &options)
        }
        "verify-hap" => {
            let Some(hap) = options.cargo_args.first() else {
                return Err("verify-hap requires a HAP path after --".to_string());
            };
            let arch = match options.arch {
                Arch::Arm64 => "arm64",
                Arch::X86_64 => "x86_64",
                Arch::Armv7a => "armv7a",
            };
            run_repo_script("verify_hap.sh", &[hap.clone(), arch.to_string()], &options)
        }
        "-h" | "--help" | "help" => Err(usage()),
        _ => Err(format!("unknown command {command:?}\n\n{}", usage())),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_mapping_matches_sdk_and_rust() {
        assert_eq!(
            Arch::parse("arm64").unwrap().rust_target(),
            "aarch64-unknown-linux-ohos"
        );
        assert_eq!(
            Arch::parse("x86_64").unwrap().clang_triple(),
            "x86_64-unknown-linux-ohos"
        );
        assert_eq!(
            Arch::parse("armv7a").unwrap().rust_target(),
            "armv7-unknown-linux-ohos"
        );
    }

    #[test]
    fn options_preserve_cargo_arguments_after_separator() {
        let options = parse_options(
            ["--arch", "x86_64", "--release", "--", "-p", "my-module"]
                .into_iter()
                .map(str::to_string),
        )
        .unwrap();
        assert_eq!(options.arch, Arch::X86_64);
        assert!(options.release);
        assert_eq!(options.cargo_args, ["-p", "my-module"]);
    }

    #[test]
    fn options_capture_reproducible_binding_inputs() {
        let options = parse_options(
            [
                "--module-descriptor",
                "entry.src.main.ets.native",
                "--ets-output",
                "generated/native.ets",
                "--library",
                "native",
            ]
            .into_iter()
            .map(str::to_string),
        )
        .unwrap();
        assert_eq!(
            options.module_descriptor.as_deref(),
            Some("entry.src.main.ets.native")
        );
        assert_eq!(
            options.ets_output,
            Some(PathBuf::from("generated/native.ets"))
        );
        assert_eq!(options.ets_library.as_deref(), Some("native"));
    }
}
