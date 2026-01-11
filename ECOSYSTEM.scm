; SPDX-License-Identifier: AGPL-3.0-or-later
; FormDB Debugger - Ecosystem Position

(ecosystem
  (version "1.0")
  (name "formdb-debugger")
  (type "tool")
  (purpose "Verified database recovery and introspection for FormDB")

  (position-in-ecosystem
    (role "Developer/DBA tool for database troubleshooting")
    (layer "Infrastructure tooling")
    (users "Database administrators, developers, auditors"))

  (related-projects
    (project "formdb"
      (relationship sibling-standard)
      (description "The core database that this debugger supports")
      (integration "Reads FormDB storage format, validates FQLdt constraints"))

    (project "fqldt"
      (relationship sibling-standard)
      (description "The type system that defines constraints")
      (integration "Uses FQLdt types in Lean 4 proofs"))

    (project "formdb-studio"
      (relationship sibling-standard)
      (description "Visual interface for FormDB")
      (integration "Debugger can be launched from Studio for recovery"))

    (project "lean4"
      (relationship dependency)
      (description "Theorem prover for recovery proofs")
      (integration "Core proof library written in Lean 4"))

    (project "idris2"
      (relationship dependency)
      (description "Dependently-typed language for REPL")
      (integration "Interactive shell and type-safe queries"))

    (project "ratatui"
      (relationship dependency)
      (description "Terminal UI library")
      (integration "TUI built with Ratatui")))

  (what-this-is
    ("A proof-carrying database debugger")
    ("A recovery tool that proves operations are safe before executing")
    ("A temporal navigation system for database states")
    ("A constraint violation analyzer with fix suggestions")
    ("An audit-grade documentation generator for data operations"))

  (what-this-is-not
    ("Not a general-purpose database GUI")
    ("Not a replacement for pg_admin or similar tools")
    ("Not a backup/restore solution (though it can verify restores)")
    ("Not a query optimizer or performance tool")))
