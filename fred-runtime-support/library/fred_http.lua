---@meta
---@diagnostic disable

fred = fred or {}

---HTTP client utilities for network requests
fred.http = {}

---@class HttpResponse
---@field status number HTTP status code (e.g. 200, 404)
---@field body string Response body text
---@field headers table<string, string> Key-value pairs of response headers

---Sends an HTTP GET request to the specified URL.
---@param url string The target URL
---@param headers? table<string, string> Optional request headers
---@return HttpResponse|nil response Response object on success, nil on error
---@return string|nil err Error message if the request failed
function fred.http.get(url, headers) end

---Sends an HTTP POST request to the specified URL with a body payload.
---@param url string The target URL
---@param body string Payload content (e.g. JSON string, raw text)
---@param headers? table<string, string> Optional request headers
---@return HttpResponse|nil response Response object on success, nil on error
---@return string|nil err Error message if the request failed
function fred.http.post(url, body, headers) end