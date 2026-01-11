-- SPDX-License-Identifier: AGPL-3.0-or-later
||| FormDB Debugger - Main Entry Point
module Main

import REPL.Core
import REPL.Commands

||| Main entry point for the FormDB Debugger
main : IO ()
main = do
  putStrLn "FormDB Debugger v0.1.0"
  putStrLn "Type 'help' for available commands, 'quit' to exit."
  putStrLn ""
  runREPL initialState
