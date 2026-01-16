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


## Structure

```
expandr/
├─ Cargo.toml            # workspace
├─ crates/
│  ├─ syntax/            # lexer + parser + AST
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ lib.rs
│  │     ├─ lexer/
│  │     ├─ parser/
│  │     ├─ ast/
│  │     └─ errors/
│  │
│  ├─ semantic/          # name resolution, type checking
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ ast_expansion/
│  │     ├─ builtins/
│  │     ├─ expand.rs
│  │     ├─ expansion_error.rs
│  │     └─ lib.rs
│  │
│  ├─ driver/            # orchestration
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs      # top-level build() function
│  │
│  └─ lsp/               # language server
│     ├─ Cargo.toml
│     └─ src/main.rs
│
└─ bin/
   └─ expandr/           # CLI binary
      ├─ Cargo.toml
      └─ src/
         ├─ cli/
         └─ main.rs
```


## Progress


### LineModeBlocks

New block mode: Currently, `[[` `]]` blocks produce template strings. I want
them to instead be `LineModeBlocks`, which parse their interior as if there was
a expr on each line:

```exr
python [[
    def to_float(x) -> float:
        y = float(x)
        return y
]]
```

Should parse equivalent to

```exr
[python
    [def to_float(x) -> float:]
        [y = float(x)]
        [return y]
]
```

For multi-line exprs, explicit brackets can be used (`[]`).


### WIP

- [ ] Line mode blocks (see above)
    - Indentation? Concatenation?
    - `is expr` as builtin
- [ ] Finish implementing repetition operators (`*`, `+`, `?`)
- [ ] Use unnamed mappings and overload only on arguments (S-Expr -> pure pattern matching)


### Extend core language

- [ ] Add number type(s) and arithmetic functions (WIP)
    - [x] Allow integer variables (resp. mappings)
    - [x] Allow implementation of built-in functions
    - [ ] Basic built-in functions (WIP)
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

