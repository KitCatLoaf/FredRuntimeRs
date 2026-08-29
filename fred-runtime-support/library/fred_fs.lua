---@meta
---@diagnostic disable

fred = fred or {}

---Filesystem utilities for Fred Runtime
fred.fs = {}

---Reads a file's contents as a string.
---@param path string The path to the file
---@return string|nil content The file content, or nil on failure
---@return string|nil err Error message if the read failed
function fred.fs.read(path) end

---Writes content to a file. Automatically creates parent directories.
---@param path string The path to write to
---@param content string The string content to write
---@return boolean success True if write succeeded
---@return string|nil err Error message if write failed
function fred.fs.write(path, content) end

---Checks if a file or directory exists.
---@param path string
---@return boolean
function fred.fs.exists(path) end

---Copies a file or an entire directory tree recursively.
---@param src string Source path
---@param dest string Destination path
---@return boolean success
---@return string|nil err
function fred.fs.copy(src, dest) end

---Creates a directory and all parent directories.
---@param path string Directory path
---@return boolean success
---@return string|nil err
function fred.fs.make(path) end

---Removes a file or directory.
---@param path string Target path
---@param recursive? boolean Defaults to true for directories
---@return boolean success
---@return string|nil err
function fred.fs.remove(path, recursive) end

---Lists entries inside a directory.
---@param path string Directory path
---@param recursive? boolean Include sub-directories
---@return table[]|nil items Array of {path, name, is_dir} objects
---@return string|nil err
function fred.fs.list(path, recursive) end