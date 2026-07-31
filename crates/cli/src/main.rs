use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const ANI_RS_REPOSITORY: &str = "https://github.com/ohos-rs/ani-rs";

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

    fn abi(self) -> &'static str {
        match self {
            Self::Arm64 => "arm64-v8a",
            Self::X86_64 => "x86_64",
            Self::Armv7a => "armeabi-v7a",
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

fn print_support() {
    println!("ani-rs support contract");
    println!("  API 23  legacy boxed primitives (--no-default-features --features api23)");
    println!("  API 24  native primitive boxing (default)");
    println!("  API 26  release/QEMU profile (--features api26)");
    println!("  header  OpenHarmony API 26 / ANI_VERSION_1");
    println!("  QEMU    ohos-qemu-vpn-20260728 / API 26");
    println!(
        "  ABI     {}, {}, {}",
        Arch::Arm64.abi(),
        Arch::X86_64.abi(),
        Arch::Armv7a.abi()
    );
}

fn scaffold_manifest(package_name: &str) -> String {
    format!(
        "[package]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\ncrate-type = [\"cdylib\"]\n\n[dependencies]\nani = {{ git = \"{ANI_RS_REPOSITORY}\", features = [\"api24\"] }}\nani-derive = {{ git = \"{ANI_RS_REPOSITORY}\" }}\n"
    )
}

fn scaffold(args: &[String]) -> Result<(), String> {
    let destination = args
        .first()
        .map(PathBuf::from)
        .ok_or_else(|| "new requires a destination path".to_string())?;
    if destination.exists() {
        return Err(format!(
            "destination already exists: {}",
            destination.display()
        ));
    }
    let raw_name = destination
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "destination must end in a valid UTF-8 project name".to_string())?;
    let package_name = raw_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();
    let library_name = package_name.replace('-', "_");
    let source_dir = destination.join("src");
    fs::create_dir_all(&source_dir).map_err(|error| {
        format!(
            "failed to create project directory {}: {error}",
            source_dir.display()
        )
    })?;
    let manifest = scaffold_manifest(&package_name);
    let source = format!(
        "use ani_derive::ani;\n\n#[ani]\npub fn add(left: i32, right: i32) -> i32 {{\n    left + right\n}}\n\n#[ani]\npub fn module_name() -> String {{\n    \"{library_name}\".to_string()\n}}\n"
    );
    fs::write(destination.join("Cargo.toml"), manifest)
        .and_then(|_| fs::write(source_dir.join("lib.rs"), source))
        .map_err(|error| format!("failed to write scaffold: {error}"))?;
    println!("created {}", destination.display());
    println!(
        "next: cd {} && ani-rs build --release",
        destination.display()
    );
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

fn usage() -> String {
    "\
ani-rs - OpenHarmony ANI build helper

Usage:
  ani-rs new PATH
  ani-rs support
  ani-rs doctor [--arch arm64|x86_64|armv7a] [--sdk PATH]
  ani-rs build  [--arch ARCH] [--sdk PATH] [--release]
                [--module-descriptor NAME] [--ets-output PATH] [--library NAME]
                [-- CARGO_ARGS...]

Repository QEMU/HAP qualification is intentionally provided by the ani-rs
source tree scripts; the published CLI has no source-tree dependency.
"
    .to_string()
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    let options = parse_options(args)?;
    match command.as_str() {
        "new" => scaffold(&options.cargo_args),
        "support" => {
            print_support();
            Ok(())
        }
        "doctor" => doctor(&options),
        "build" => build(&options),
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
        assert_eq!(Arch::Arm64.abi(), "arm64-v8a");
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

    #[test]
    fn scaffold_uses_the_project_repository_instead_of_a_crates_io_name_collision() {
        let manifest = scaffold_manifest("demo-addon");
        assert!(manifest.contains("name = \"demo-addon\""));
        assert!(manifest.contains(
            "ani = { git = \"https://github.com/ohos-rs/ani-rs\", features = [\"api24\"] }"
        ));
        assert!(manifest.contains("ani-derive = { git = \"https://github.com/ohos-rs/ani-rs\" }"));
        assert!(!manifest.contains("ani = { version"));
    }
}
