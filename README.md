# AION — AI-Native Programming Language & Runtime (WIP)

AION is a modern **AI-native programming language and runtime**, built entirely from scratch in **Rust**.  
It is designed as a long-term experimental platform to explore:

- custom language design  
- lexing, parsing, and AST generation  
- interpreter and bytecode virtual machine design  
- memory management + garbage collection  
- async concurrency and event loops  
- native Rust extensions & plugin architecture  
- vector & matrix operations for AI workloads  
- distributed execution in future phases  
- WebAssembly backend (planned)

AION is **not** intended to compete with established languages.  
It is a **research & learning project** to understand compilers, runtimes, AI execution models, and systems programming deeply.

---

# 🚀 Current Working Features (Phase 1 Complete)

AION already supports real executable code through its interpreter and REPL.

### ✔ Variable bindings  
let x = 10;
let y = x + 20;
y;


### ✔ Arithmetic expressions  
5 + 3 * 2;
(10 - 4) / 2;
-5;


### ✔ Comparison operators  
5 < 10;
10 > 3;
5 == 5;
5 != 3;


### ✔ Prefix operators  
!true; // coming soon
!(5 < 10);
-(-10);


### ✔ Expression statements  
10 * 2 + 5;


### ✔ Return statements  
return 10;
return x + 5;


These features already allow AION to evaluate meaningful programs.

---

# 🖥 Interactive REPL
AION includes a fully functional REPL.  
Run it with:
    cargo run

### Example session:
AION REPL — Type Ctrl+C to exit
aion> let x = 10;
null
aion> x;
10
aion> 5 + 3 * 2;
11
aion> !(5 < 10);
false
aion> let y = x + 20;
null
aion> y;
30


This REPL uses the live interpreter and retains variable assignments across commands.

---

# 🧠 Architecture Overview (Phase 1)

AION currently implements:

| Component        | Status | Description |
|------------------|--------|-------------|
| **Lexer**        | ✅ Done | Tokenizes raw input |
| **Parser**       | ✅ Done | Pratt parser with precedence handling |
| **AST**          | ✅ Done | Represents program structure |
| **Interpreter**  | ✅ Done | Evaluates AST dynamically |
| **Environment**  | ✅ Done | Stores variable bindings |
| **REPL**         | ✅ Done | Full interactive shell |

This is equivalent to building the core of a small JavaScript/Python/Lua-like interpreter.

---

# 📁 Project Structure
aion-lang/
├── src/
│ ├── lexer/
│ ├── parser/
│ ├── ast/
│ ├── interpreter/
│ ├── lib.rs
│ └── main.rs
├── tests/
├── README.md
└── Cargo.toml


This structure will expand significantly in future phases.

---

# 🛣️ Roadmap

### **Phase 1 — Interpreter & REPL** ✔ (Completed)
- Lexer  
- Parser  
- AST  
- Expression evaluator  
- Variable environment  
- REPL  

### **Phase 2 — Bytecode Virtual Machine**
- Compile AST → bytecode  
- Stack-based VM  
- Faster execution vs interpreter  

### **Phase 3 — Memory Model**
- Heap allocation  
- Object representation  
- Mark-and-Sweep garbage collector  

### **Phase 4 — Functions & Closures**
- `fn(x, y) { x + y }`  
- Lexical scoping  
- First-class functions  

### **Phase 5 — Collections**
- Arrays  
- Hash maps  
- Strings  

### **Phase 6 — Built-in Functions**
- `len()`, `push()`, `first()`  
- REPL utilities  

### **Phase 7 — AI Extensions**
- `ai.ask("...")`  
- `ai.summarize(text)`  
- AI-native operators  
- Deterministic + nondeterministic execution modes  

### **Phase 8 — Concurrency**
- async/await  
- Event loop  
- Scheduler  

### **Phase 9 — WASM Backend**
- Compile AION → WASM  
- Embed WASM inside AION  

This roadmap will evolve as AI and systems engineering progress.

---

# 🛠 Technology

- **Rust** (safe + fast + ideal for system-level work)  
- **Cargo** (build system & dependency manager)  
- **Rust 2021 Edition**

---

# 📜 License

MIT License (you may change later)

---

# 🤝 Contributing

This is currently a personal educational project.  
Public contributions and discussions will open later.

---

# 🌟 Status: Phase 1 complete — full interpreter + REPL working.


