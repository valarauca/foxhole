//!Partial Ast
//!
//! A two phase type check is preformed off due to
//! how homomorphisms are declared. This adds some
//! complications to the AST -> SSA conversion.
//!
//! The phases are roughly:
//!
//! 1. Parse Tree (raw LR-Parser output)
//! 2. Partial AST
//!     - Names validated as existing
//!     - Templates expanded
//!     - Partial type checking
//! 3. Full AST
//!     - Types validated
//! 4. IR
//! 5. Optimization
//! 5. Execution

mod phase1;
