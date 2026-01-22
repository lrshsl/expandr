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


## Structure

```
expandr/
├─ crates
│  ├─ syntax            # lexer + parser + AST
│  ├─ semantic          # name resolution, type checking
│  ├─ driver            # orchestration
│  └─ lsp               # language server
│
└─ bin/expandr          # CLI binary
```


## Progress


### Active WIP

- [x] Line mode blocks
    - Indentation?
    - [ ] `is expr` as builtin - degrade to soft-keyword
- [ ] Finish implementing repetition operators (`*`, `+`, `?`)
- [ ] Use unnamed mappings and overload only on arguments (S-Expr -> pure pattern matching)


### Niceties

- Lsp
- Shell completion for cli
- TS grammar

### Extend core language

- [ ] Basic built-in functions (WIP)
    - [ ] is expr
    - [ ] arithmetic
- [ ] Lists
- [ ] Namespaces / modules
    - [x] Importing
    - [ ] `pub` / `priv`
    - [ ] Explicit interfaces?
- [ ] Pattern matching
    - [ ] Closures
        - lisp-like quasi-quoting?
    - [ ] `_` special context variable
- Optimizations (later)
    - [ ] Interning symbol names?


### Implement libraries

- LLVM backed stdlib
- Py translator (fun)

