; SPDX-License-Identifier: AGPL-3.0-or-later
; FormDB Debugger - Project State

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2025-01-11")
    (updated "2026-01-11")
    (project "formdb-debugger")
    (repo "https://github.com/hyperpolymath/formdb-debugger"))

  (project-context
    (name "FormDB Debugger")
    (tagline "Proof-carrying database recovery and introspection")
    (tech-stack
      (primary "Lean 4" "Idris 2")
      (secondary "Rust")
      (ui "Ratatui")))

  (current-position
    (phase "scaffolding-complete")
    (overall-completion 35)
    (components
      (specification 100 "SPEC.adoc complete")
      (core-proofs 70 "Types and proof stubs implemented")
      (idris-repl 60 "REPL framework and commands scaffolded")
      (rust-adapters 50 "Adapter structure with trait definitions")
      (tui 40 "Widget scaffolds implemented"))
    (working-features
      ("Lean 4 lake project builds"
       "Core type definitions (Schema, Constraint, Query)"
       "State types (Snapshot, Delta, Transaction)"
       "Proof stubs (Lossless, FDPreserving, Rollback)"
       "Idris 2 package structure"
       "REPL core loop"
       "Recovery plan types"
       "Rust workspace with 3 adapter crates"
       "PostgreSQL pg_catalog queries"
       "FormDB journal format parsing"
       "Ratatui widget scaffolds")))

  (route-to-mvp
    (milestone "Phase 1: Core Proof Library"
      (status "in-progress")
      (items
        ("Define Schema/Table/Column types in Lean 4" . done)
        ("Define Snapshot and Delta types" . done)
        ("Implement constraint checking proofs" . stub)
        ("Implement lossless join proofs" . stub)
        ("Implement reversibility proofs" . stub)))
    (milestone "Phase 2: Database Adapters"
      (status "in-progress")
      (items
        ("PostgreSQL adapter (pg_catalog)" . scaffold)
        ("FormDB native adapter" . scaffold)
        ("SQLite adapter for testing" . scaffold)
        ("WAL parsing" . not-started)
        ("Transaction log extraction" . not-started)))
    (milestone "Phase 3: Idris 2 REPL"
      (status "in-progress")
      (items
        ("REPL framework" . done)
        ("Schema loading" . stub)
        ("Query execution with type checking" . not-started)
        ("Recovery plan generation" . scaffold)
        ("Proof display" . not-started)))
    (milestone "Phase 4: Terminal UI"
      (status "in-progress")
      (items
        ("Ratatui TUI scaffold" . done)
        ("Timeline visualization" . scaffold)
        ("Constraint violation browser" . scaffold)
        ("Recovery plan wizard" . scaffold))))

  (blockers-and-issues
    (critical ())
    (high-priority
      ("Lean 4 proofs need actual theorem implementations"))
    (medium-priority
      ("Idris 2 dependent types need real constraints")
      ("Rust adapters need actual database connections"))
    (low-priority
      ("TUI needs styling and keyboard navigation")))

  (critical-next-actions
    (immediate
      ("Implement actual Lean 4 proofs for Lossless theorem")
      ("Add real PostgreSQL connection to adapter"))
    (this-week
      ("Complete constraint satisfaction proofs")
      ("Test Idris 2 REPL with sample schema"))
    (this-month
      ("End-to-end: load schema -> detect violation -> generate recovery plan")))

  (session-history
    (snapshot "2026-01-11"
      (accomplishments
        ("Created full project scaffold (41 files, 1918 lines)")
        ("Lean 4 core library: Types, State, Proofs modules")
        ("Idris 2 REPL: Core, Commands, Inspector, Recovery modules")
        ("Rust adapters: postgres, formdb, sqlite crates")
        ("Ratatui TUI: timeline, constraint_tree, recovery_plan, proof_viewer widgets")
        ("Added formdb-debugger to all 5 FormDB ecosystem repos")
        ("Pushed all changes to GitHub")))
    (snapshot "2025-01-11"
      (accomplishments
        ("Wrote comprehensive SPEC.adoc")
        ("Created README.adoc")
        ("Set up project structure")))))
