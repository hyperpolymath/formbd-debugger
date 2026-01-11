-- SPDX-License-Identifier: AGPL-3.0-or-later
||| REPL Core - Interactive debugging session state
module REPL.Core

import Data.List

||| Database column data types
public export
data DataType = DInt | DText | DBool | DTimestamp | DUUID | DJson | DBytes

||| A database column
public export
record Column where
  constructor MkColumn
  name : String
  dtype : DataType
  nullable : Bool

||| A database table
public export
record Table where
  constructor MkTable
  name : String
  columns : List Column
  primaryKey : List String

||| A database schema
public export
record Schema where
  constructor MkSchema
  name : String
  tables : List Table

||| Debug session state
public export
record DebugState where
  constructor MkDebugState
  connected : Bool
  schema : Maybe Schema
  currentTable : Maybe String
  history : List String

||| Initial state with no connection
public export
initialState : DebugState
initialState = MkDebugState False Nothing Nothing []

||| Run the REPL loop
public export
runREPL : DebugState -> IO ()
runREPL state = do
  putStr "formdb-debug> "
  line <- getLine
  case line of
    "quit" => putStrLn "Goodbye!"
    "exit" => putStrLn "Goodbye!"
    cmd => do
      putStrLn $ "Command: " ++ cmd
      runREPL (record { history $= (cmd ::) } state)
