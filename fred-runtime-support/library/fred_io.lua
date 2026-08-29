---@meta
---@diagnostic disable

fred = fred or {}

---Terminal input/output utilities for Fred Runtime
fred.io = {}

---Prompts the user for input in the console and returns the entered string.
---@param prompt? string Optional message to display before reading input
---@return string input The line entered by the user (trimmed of trailing newline)
function fred.io.read(prompt) end

---Prints one or more values to stdout followed by a newline.
---@param ... any Values to print
function fred.io.print(...) end

---Prints a formatted string to stdout (similar to printf).
---@param format_str string Formatted string template
---@param ... any Arguments to format into the template
function fred.io.printf(format_str, ...) end

---Clears the current terminal window screen.
function fred.io.clear() end