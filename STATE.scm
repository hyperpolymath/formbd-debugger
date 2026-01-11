; SPDX-License-Identifier: AGPL-3.0-or-later
; FormDB Debugger - Project State

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2025-01-11")
    (updated "2025-01-11")
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
    (phase "specification")
    (overall-completion 5)
    (components
      (specification 100 "SPEC.adoc complete")
      (core-proofs 0 "Not started")
      (idris-repl 0 "Not started")
      (rust-adapters 0 "Not started")
      (tui 0 "Not started"))
    (working-features
      ()))

  (route-to-mvp
    (milestone "Phase 1: Core Proof Library"
      (items
        ("Define Schema/Table/Column types in Lean 4")
        ("Define Snapshot and Delta types")
        ("Implement constraint checking proofs")
        ("Implement lossless join proofs")
        ("Implement reversibility proofs")))
    (milestone "Phase 2: Database Adapters"
      (items
        ("PostgreSQL adapter (pg_catalog)")
        ("FormDB native adapter")
        ("SQLite adapter for testing")
        ("WAL parsing")
        ("Transaction log extraction")))
    (milestone "Phase 3: Idris 2 REPL"
      (items
        ("REPL framework")
        ("Schema loading")
        ("Query execution with type checking")
        ("Recovery plan generation")
        ("Proof display")))
    (milestone "Phase 4: Terminal UI"
      (items
        ("Ratatui TUI scaffold")
        ("Timeline visualization")
        ("Constraint violation browser")
        ("Recovery plan wizard"))))

  (blockers-and-issues
    (critical ())
    (high-priority ())
    (medium-priority ())
    (low-priority ()))

  (critical-next-actions
    (immediate
      ("Create GitHub repo")
      ("Set up Lean 4 lake project"))
    (this-week
      ("Define core Schema types in Lean 4"))
    (this-month
      ("Complete Phase 1 proof library")))

  (session-history
    (snapshot "2025-01-11"
      (accomplishments
        ("Wrote comprehensive SPEC.adoc")
        ("Created README.adoc")
        ("Set up project structure")))))
