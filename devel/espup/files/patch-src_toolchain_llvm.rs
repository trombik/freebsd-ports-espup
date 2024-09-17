--- src/toolchain/llvm.rs.orig	2024-07-18 11:41:37 UTC
+++ src/toolchain/llvm.rs
@@ -55,6 +55,7 @@ impl Llvm {
                 HostTriple::X86_64AppleDarwin => "x86_64-apple-darwin",
                 HostTriple::X86_64UnknownLinuxGnu => "x86_64-linux-gnu",
                 HostTriple::Aarch64UnknownLinuxGnu => "aarch64-linux-gnu",
+                HostTriple::X86_64UnknownFreeBSD => "x86_64-portbld-freebsd",
                 HostTriple::X86_64PcWindowsMsvc | HostTriple::X86_64PcWindowsGnu => {
                     "x86_64-w64-mingw32"
                 }
@@ -66,6 +67,7 @@ impl Llvm {
                 HostTriple::X86_64AppleDarwin => "macos",
                 HostTriple::X86_64UnknownLinuxGnu => "linux-amd64",
                 HostTriple::Aarch64UnknownLinuxGnu => "linux-arm64",
+                HostTriple::X86_64UnknownFreeBSD => "freebsd-amd64",
                 HostTriple::X86_64PcWindowsMsvc | HostTriple::X86_64PcWindowsGnu => "win64",
             };
             arch.to_string()
