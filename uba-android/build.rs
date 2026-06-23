use std::env;
use std::path::PathBuf;

const ANDROID_ENV_VARS: [&str; 4] = [
    "ANDROID_HOME",
    "ANDROID_SDK_ROOT",
    "ANDROID_NDK_HOME",
    "JAVA_HOME",
];

fn main() {
    if cfg!(feature = "build-checks") {
        for var in ANDROID_ENV_VARS {
            println!("cargo:rerun-if-env-changed={var}");
        }
        println!("cargo:rerun-if-env-changed=TARGET");

        // **************************** Target Check **************************** //
        let target = env::var("TARGET").expect("TARGET is not set");
        if !target.contains("android") {
            panic!(
                "error: uba-android must be built for Android, not `{target}`.\n\
             Try: cargo build -p uba-android --target aarch64-linux-android"
            );
        }

        let linker_env = format!(
            "CARGO_TARGET_{}_LINKER",
            target.to_uppercase().replace('-', "_")
        );
        println!("cargo:rerun-if-env-changed={linker_env}");

        // *************************** Android Check **************************** //
        if env_var_path("ANDROID_HOME")
            .or_else(|| env_var_path("ANDROID_SDK_ROOT"))
            .filter(|p| p.is_dir())
            .is_none()
        {
            panic!(
                "Android SDK not found. Make sure to set either to your SDK root \
             (the directory that contains `platforms/`)",
            );
        }

        let linker = env_var_path(&linker_env).unwrap_or_else(|| panic!("{linker_env} not set"));
        if !linker.is_file() {
            panic!(
                "error: {linker_env} is set but not a file: {}\n\
             Point it at the NDK's {target}*-clang binary.",
                linker.display()
            );
        }

        // ***************************** Java Check ***************************** //
        let java_home = env_var_path("JAVA_HOME").expect(
            "JAVA_HOME is not set.\n \
         Slint's Android backend compiles Java helpers at build time and needs a JDK.\n\
         Example:\n\
         export JAVA_HOME=/usr/lib/jvm/default",
        );

        let javac = java_home.join("bin").join("javac");
        if !javac.is_file() {
            panic!(
                "error: javac not found at {}.\n\
             JAVA_HOME must point at a JDK installation (not just a JRE).\n\
             Example:\n\
             export JAVA_HOME=/usr/lib/jvm/default",
                javac.display()
            );
        }
    }

    slint_build::compile("../ui/app.slint").expect("Slint build failed");
}

fn env_var_path(name: &str) -> Option<PathBuf> {
    env::var(name)
        .ok()
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
}
