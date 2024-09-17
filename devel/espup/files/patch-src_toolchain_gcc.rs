--- src/toolchain/gcc.rs.orig	2024-07-18 11:41:37 UTC
+++ src/toolchain/gcc.rs
@@ -133,6 +133,7 @@ fn get_arch(host_triple: &HostTriple) -> Result<&str> 
         HostTriple::Aarch64AppleDarwin => Ok("aarch64-apple-darwin"),
         HostTriple::X86_64UnknownLinuxGnu => Ok("x86_64-linux-gnu"),
         HostTriple::Aarch64UnknownLinuxGnu => Ok("aarch64-linux-gnu"),
+        HostTriple::X86_64UnknownFreeBSD => Ok("x86_64-portbld-freebsd"),
         HostTriple::X86_64PcWindowsMsvc | HostTriple::X86_64PcWindowsGnu => {
             Ok("x86_64-w64-mingw32")
         }
