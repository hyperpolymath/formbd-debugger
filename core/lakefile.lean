-- SPDX-License-Identifier: AGPL-3.0-or-later
-- FormDB Debugger Core Library

import Lake
open Lake DSL

package «formdb-debugger-core» where
  leanOptions := #[
    ⟨`pp.unicode.fun, true⟩,
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib «FormDBDebugger» where
  roots := #[`FormDBDebugger]
