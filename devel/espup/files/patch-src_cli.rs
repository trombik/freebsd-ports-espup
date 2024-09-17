--- src/cli.rs.orig	2024-07-18 11:41:37 UTC
+++ src/cli.rs
@@ -17,7 +17,7 @@ pub struct InstallOpts {
 #[derive(Debug, Parser)]
 pub struct InstallOpts {
     /// Target triple of the host.
-    #[arg(short = 'd', long, value_parser = ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu" , "x86_64-apple-darwin" , "aarch64-apple-darwin"])]
+    #[arg(short = 'd', long, value_parser = ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu" , "x86_64-apple-darwin" , "aarch64-apple-darwin", "x86_64-unknown-freebsd"])]
     pub default_host: Option<String>,
     /// Install Espressif RISC-V toolchain built with croostool-ng
     ///
