 [![License](https://img.shields.io/badge/License-EPL_1.0-red.svg)](https://opensource.org/licenses/EPL-1.0)

 # Expandr

 > A language that compiles itself when you run it.


The idea of this project is to create .. something? .. that can be thought of as:
- a templating engine
- a macro system
- a programming language

Where each source code file produces a string when _expanded_. Possible use cases include:

- Adding variables / functions to config files
- Generate verbose / repeating code
- Generate C code, MLIR (or any intermediate representation) or directly an executable

You can find examples in the [examples'](examples/) directory.


## Progress

- [ ] Finish implementing repetition operators (`*`, `+`, `?`)
- [ ] Implement scopes
    - [ ] function scope `>` global scope
- [ ] Identifiers as expressions
    - either take their string value
    - or resolve (lookup) directly (without `[]`)


Extend core language:

- [ ] Add number type(s) and arithmetic functions (WIP)
    - [x] Allow integer variables (resp. mappings)
    - [ ] Basic built-in functions (WIP)
- [ ] Namespaces / modules
    - [ ] Importing
    - [ ] `pub` / `priv`
    - [ ] Explicit interfaces?
- [ ] Pattern matching
    - [ ] Closures
        - lisp-like quasi-quoting?
    - [ ] `_` special context variable
- Optimizations (later)

