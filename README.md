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
- Generate C code / assembly / directly an executable

You can find examples in the [examples](examples/) directory.


## Progress

- [ ] Finish implementing repetition operators (`*`, `+`, `?`)
- [ ] Implement scopes
    - [ ] function scope `>` global scope


Extend core language:

- [ ] Add number type(s) and arithmetic functions (WIP)
- [ ] Namespaces / modules
    - [ ] Importing
    - [ ] `pub` / `priv`
    - [ ] Explicit interfaces?
- [ ] Closures
- [ ] Pattern matching
- Optimizations (later)

