--- src/host_triple.rs.orig	2024-07-18 11:41:37 UTC
+++ src/host_triple.rs
@@ -27,6 +27,9 @@ pub enum HostTriple {
     /// ARM64 macOS
     #[strum(serialize = "aarch64-apple-darwin")]
     Aarch64AppleDarwin,
+    /// 64-bit FreeBSD
+    #[strum(serialize = "x86_64-unknown-freebsd")]
+    X86_64UnknownFreeBSD,
 }
 
 /// Parse the host triple if specified, otherwise guess it.
@@ -70,6 +73,10 @@ mod tests {
             get_host_triple(Some("aarch64-apple-darwin".to_string())),
             Ok(HostTriple::Aarch64AppleDarwin)
         ));
+        assert!(matches!(
+            get_host_triple(Some("x86_64-unknown-freebsd".to_string())),
+            Ok(HostTriple::X86_64UnknownFreeBSD)
+        ));
 
         assert!(get_host_triple(Some("some-fake-triple".to_string())).is_err());
 
@@ -103,6 +110,11 @@ mod tests {
         assert!(matches!(
             get_host_triple(None),
             Ok(HostTriple::Aarch64AppleDarwin)
+        ));
+        #[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
+        assert!(matches!(
+            get_host_triple(None),
+            Ok(HostTriple::X86_64UnknownFreeBSD)
         ));
     }
 }
