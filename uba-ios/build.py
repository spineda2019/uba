#!/usr/bin/env python3

import os
import sys
import subprocess
from pathlib import Path


def main() -> int:
    print(80 * "#")
    print("# Script stdout")
    print(80 * "#")

    print(f"Arguments passed to {__file__}")
    for (i, arg) in enumerate(sys.argv):
        print(f"\targ{i}: {arg}")

    env: dict[str, str] = dict(os.environ)
    scriptroot: str = os.path.dirname(os.path.abspath(__file__))

    print("patching up PATH")

    # Fix up PATH to work around
    # https://github.com/rust-lang/rust/issues/80817
    env["PATH"] = (
        "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:"
        f"{env.get('PATH', '')}:"
        f"{env.get('HOME', '')}/.cargo/bin"
    )

    optimization_args: list[str] = []

    cargo_profile: str = "debug"
    if env["CONFIGURATION"] != "Debug":
        optimization_args.append("--release")
        cargo_profile = "release"

    print(f"Build optimization: {cargo_profile}")

    # Build with debug info so a dSYM can be produced.
    env.setdefault("CARGO_PROFILE_RELEASE_DEBUG", "1")

    # Make Cargo output cache files in Xcode's directories.
    derived_file_dir: str = env["DERIVED_FILE_DIR"]
    env["CARGO_TARGET_DIR"] = os.path.join(derived_file_dir, "cargo")

    print(f"CARGO_TARGET_DIR: {env['CARGO_TARGET_DIR']}")

    # Forward CI build number if present.
    ci_build_number: str | None = env.get("CI_BUILD_NUMBER")
    if ci_build_number:
        env["SLINT_BUILD_NUMBER"] = ci_build_number

    is_simulator: bool = env.get("LLVM_TARGET_TRIPLE_SUFFIX") == "-simulator"

    print(f"Is simulator: {is_simulator}")

    archs: list[str] = env["ARCHS"].split()
    executables: list[str] = []

    for arch in archs:
        print(f"building for arch: {arch}")
        if arch == "arm64":
            if is_simulator:
                cargo_target: str = "aarch64-apple-ios-sim"
            else:
                cargo_target = "aarch64-apple-ios"

        elif arch == "x86_64":
            env["CFLAGS_x86_64_apple_ios"] = "-target x86_64-apple-ios"
            cargo_target = "x86_64-apple-ios"

        else:
            raise RuntimeError(f"Unsupported architecture: {arch}")

        args: list[str] = ["cargo",
                           "build",
                           "--target",
                           cargo_target]
        if len(optimization_args) > 0:
            args.extend(optimization_args)
        if len(sys.argv) > 1:
            args.extend(sys.argv[1:])
        subprocess.run(args,
                       check=True,
                       env=env,
                       cwd=os.path.join(scriptroot, ".."))

        executable: str = os.path.join(
            derived_file_dir,
            "cargo",
            cargo_target,
            cargo_profile,
            "uba")
        executables.append(executable)

    target_build_dir: str = env["TARGET_BUILD_DIR"]
    executable_path: str = env["EXECUTABLE_PATH"]

    print(f"target build dir: {target_build_dir}")
    print(f"executable path: {executable_path}")

    lipo = ["lipo",
            "-create",
            "-output",
            os.path.join(target_build_dir, executable_path),
            *executables]

    print("Lipo args: ")
    for (i, arg) in enumerate(lipo):
        print(f"\targ{i}: {arg}")

    subprocess.run(lipo, env=env)

    dwarf_dsym_folder: str | None = env.get("DWARF_DSYM_FOLDER_PATH")
    dwarf_dsym_name: str | None = env.get("DWARF_DSYM_FILE_NAME")

    if dwarf_dsym_folder and dwarf_dsym_name:
        Path(dwarf_dsym_folder).mkdir(parents=True, exist_ok=True)

        subprocess.run(["dsymutil",
                        os.path.join(target_build_dir, executable_path),
                        "-o",
                        os.path.join(dwarf_dsym_folder, dwarf_dsym_name)],
                       env=env)

    code_signing_allowed: str = env.get("CODE_SIGNING_ALLOWED", "YES")
    expanded_identity: str = env.get("EXPANDED_CODE_SIGN_IDENTITY", "")

    if (
        not is_simulator
        and code_signing_allowed != "NO"
        and expanded_identity
    ):
        entitlements_file: Path = (
            Path(env["TARGET_TEMP_DIR"])
            / f"{env['PRODUCT_NAME']}.app.xcent"
        )

        cmd: list[str] = ["codesign",
                          "--force",
                          "--sign",
                          expanded_identity]

        if (entitlements_file.is_file()
                and entitlements_file.stat().st_size > 0):
            cmd.extend(["--entitlements", str(entitlements_file)])

        cmd.append(str(Path(target_build_dir) / executable_path))

        subprocess.run(cmd, env=env)

    return 0


if __name__ == "__main__":
    ec: int = main()
    sys.exit(ec)
