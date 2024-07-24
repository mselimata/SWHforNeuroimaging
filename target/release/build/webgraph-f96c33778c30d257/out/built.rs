//
// EVERYTHING BELOW THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
#[doc=r#"The Continuous Integration platform detected during compilation."#]
#[allow(dead_code)]
pub const CI_PLATFORM: Option<&str> = None;
#[doc=r#"The full version."#]
#[allow(dead_code)]
pub const PKG_VERSION: &str = r"0.1.0";
#[doc=r#"The major version."#]
#[allow(dead_code)]
pub const PKG_VERSION_MAJOR: &str = r"0";
#[doc=r#"The minor version."#]
#[allow(dead_code)]
pub const PKG_VERSION_MINOR: &str = r"1";
#[doc=r#"The patch version."#]
#[allow(dead_code)]
pub const PKG_VERSION_PATCH: &str = r"0";
#[doc=r#"The pre-release version."#]
#[allow(dead_code)]
pub const PKG_VERSION_PRE: &str = r"";
#[doc=r#"A colon-separated list of authors."#]
#[allow(dead_code)]
pub const PKG_AUTHORS: &str = r"";
#[doc=r#"The name of the package."#]
#[allow(dead_code)]
pub const PKG_NAME: &str = r"webgraph";
#[doc=r#"The description."#]
#[allow(dead_code)]
pub const PKG_DESCRIPTION: &str = r"A Rust port of the WebGraph framework (http://webgraph.di.unimi.it/).";
#[doc=r#"The homepage."#]
#[allow(dead_code)]
pub const PKG_HOMEPAGE: &str = r"";
#[doc=r#"The license."#]
#[allow(dead_code)]
pub const PKG_LICENSE: &str = r"Apache-2.0 OR LGPL-2.1-or-later";
#[doc=r#"The source repository as advertised in Cargo.toml."#]
#[allow(dead_code)]
pub const PKG_REPOSITORY: &str = r"https://github.com/vigna/webgraph-rs/";
#[doc=r#"The target triple that was being compiled for."#]
#[allow(dead_code)]
pub const TARGET: &str = r"x86_64-unknown-linux-gnu";
#[doc=r#"The host triple of the rust compiler."#]
#[allow(dead_code)]
pub const HOST: &str = r"x86_64-unknown-linux-gnu";
#[doc=r#"`release` for release builds, `debug` for other builds."#]
#[allow(dead_code)]
pub const PROFILE: &str = r"release";
#[doc=r#"The compiler that cargo resolved to use."#]
#[allow(dead_code)]
pub const RUSTC: &str = r"/home/matay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc";
#[doc=r#"The documentation generator that cargo resolved to use."#]
#[allow(dead_code)]
pub const RUSTDOC: &str = r"/home/matay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc";
#[doc=r#"Value of OPT_LEVEL for the profile used during compilation."#]
#[allow(dead_code)]
pub const OPT_LEVEL: &str = r"3";
#[doc=r#"The parallelism that was specified during compilation."#]
#[allow(dead_code)]
pub const NUM_JOBS: u32 = 20;
#[doc=r#"Value of DEBUG for the profile used during compilation."#]
#[allow(dead_code)]
pub const DEBUG: bool = true;
#[doc=r#"The features that were enabled during compilation."#]
#[allow(dead_code)]
pub const FEATURES: [&str; 2] = ["CLI", "DEFAULT"];
#[doc=r#"The features as a comma-separated string."#]
#[allow(dead_code)]
pub const FEATURES_STR: &str = r"CLI, DEFAULT";
#[doc=r#"The features as above, as lowercase strings."#]
#[allow(dead_code)]
pub const FEATURES_LOWERCASE: [&str; 2] = ["cli", "default"];
#[doc=r#"The feature-string as above, from lowercase strings."#]
#[allow(dead_code)]
pub const FEATURES_LOWERCASE_STR: &str = r"cli, default";
#[doc=r#"The output of `/home/matay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc -V`"#]
#[allow(dead_code)]
pub const RUSTC_VERSION: &str = r"rustc 1.79.0 (129f3b996 2024-06-10)";
#[doc=r#"The output of `/home/matay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc -V`; empty string if `/home/matay/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc -V` failed to execute"#]
#[allow(dead_code)]
pub const RUSTDOC_VERSION: &str = r"rustdoc 1.79.0 (129f3b996 2024-06-10)";
#[doc=r#"The target architecture, given by `CARGO_CFG_TARGET_ARCH`."#]
#[allow(dead_code)]
pub const CFG_TARGET_ARCH: &str = r"x86_64";
#[doc=r#"The endianness, given by `CARGO_CFG_TARGET_ENDIAN`."#]
#[allow(dead_code)]
pub const CFG_ENDIAN: &str = r"little";
#[doc=r#"The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`."#]
#[allow(dead_code)]
pub const CFG_ENV: &str = r"gnu";
#[doc=r#"The OS-family, given by `CARGO_CFG_TARGET_FAMILY`."#]
#[allow(dead_code)]
pub const CFG_FAMILY: &str = r"unix";
#[doc=r#"The operating system, given by `CARGO_CFG_TARGET_OS`."#]
#[allow(dead_code)]
pub const CFG_OS: &str = r"linux";
#[doc=r#"The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`."#]
#[allow(dead_code)]
pub const CFG_POINTER_WIDTH: &str = r"64";
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_VERSION` contains HEAD's tag. The short commit id is used if HEAD is not tagged."#]
#[allow(dead_code)]
pub const GIT_VERSION: Option<&str> = Some("4d83236");
#[doc=r#"If the repository had dirty/staged files."#]
#[allow(dead_code)]
pub const GIT_DIRTY: Option<bool> = Some(false);
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_HEAD_REF` contains full name to the reference pointed to by HEAD (e.g.: `refs/heads/master`). If HEAD is detached or the branch name is not valid UTF-8 `None` will be stored.
"#]
#[allow(dead_code)]
pub const GIT_HEAD_REF: Option<&str> = Some("refs/heads/master");
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH` contains HEAD's full commit SHA-1 hash."#]
#[allow(dead_code)]
pub const GIT_COMMIT_HASH: Option<&str> = Some("4d8323683fcb5953c0f9c097e50432fc826abea6");
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH_SHORT` contains HEAD's short commit SHA-1 hash."#]
#[allow(dead_code)]
pub const GIT_COMMIT_HASH_SHORT: Option<&str> = Some("4d83236");
#[doc=r#"The build time in RFC2822, UTC."#]
#[allow(dead_code)]
pub const BUILT_TIME_UTC: &str = r"Wed, 24 Jul 2024 14:30:01 +0000";
//
// EVERYTHING ABOVE THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
