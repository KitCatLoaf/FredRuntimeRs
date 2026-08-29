---@meta
---@diagnostic disable

fred = fred or {}

---Time and timing utilities for Fred Runtime
fred.time = {}

---Pauses script execution for the specified number of milliseconds.
---@param ms number Milliseconds to sleep
function fred.time.sleep(ms) end

---Returns the current Unix timestamp in seconds since epoch.
---@return number timestamp
function fred.time.now() end

---Returns high-resolution elapsed time in milliseconds (useful for benchmarking/timing code execution).
---@return number milliseconds
function fred.time.ticks() end

---Returns a formatted date/time string based on local system time.
---@param format_str? string Optional strftime format (e.g. "%Y-%m-%d %H:%M:%S"). Defaults to ISO timestamp.
---@return string formatted_date
function fred.time.date(format_str) end