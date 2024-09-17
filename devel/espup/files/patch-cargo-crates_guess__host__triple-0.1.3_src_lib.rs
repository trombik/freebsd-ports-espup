--- cargo-crates/guess_host_triple-0.1.3/src/lib.rs.orig	2024-09-17 11:09:04 UTC
+++ cargo-crates/guess_host_triple-0.1.3/src/lib.rs
@@ -93,6 +93,7 @@ impl Architecture {
             b"mips64" => Mips64,
             b"powerpc" => PowerPc,
             b"powerpc64" => PowerPc64,
+            b"amd64" => X86_64,
             _ => {
                 warn!("uname returned unrecognised machine {:?}", machine);
                 Unknown
