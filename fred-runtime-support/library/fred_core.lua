---@meta
---@diagnostic disable

fred = fred or {}

---Returns the current version string of Fred Runtime.
---@return string
function fred.version() end

---The host operating system platform ("windows", "linux", "macos").
---@return string
function fred.platform() end

---Returns colored text string for terminal printing.
---@param text string
---@param color "red"|"green"|"yellow"|"blue"|"magenta"|"cyan"
---@return string
function fred.color(text, color) end