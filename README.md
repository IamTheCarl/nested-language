NL (Nested Language) is designed to enable applications to run external code in a safe and sandboxed form which is
convenient for both the host and client languages. Unlike most other languages used to fill this niche, NL is not a
dynamically interpreted scripting language. Instead, it is designed to be statically evaluated by a compiler and IDEs.

This packages contains only the compiler, which is still in development. This package cannot execute its compiled forms
of the language. The compiled forms are ether to be serialized and loaded at another time and place, or to be given to
an interpreter for execution, or a re-interpreter to convert to a different run format, such as a jit compiler or native
binary.