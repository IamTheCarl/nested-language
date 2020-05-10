NL (Nested Language) is designed to enable applications to run external code in a safe and sandboxed form which is
convenient for both the host and client languages. Unlike most other languages used to fill this niche, NL is not a
dynamically interpreted scripting language. Instead, it is designed to be compiled into a static binary form.

Nested is designed to be thread safe, and thus avoids global state as much as possible. Only non-modifiable code is
stored in a global state, which once loaded is read only.