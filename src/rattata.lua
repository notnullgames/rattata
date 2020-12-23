local socket = require 'socket'
local ssl = require 'ssl'
local http_request = require "http.request"

local server = assert(socket.bind("*", 0))
local ip, port = server:getsockname()

print("Please telnet to localhost on port " .. port .. ". Give it a URL to download & run.")


function run(url)
  local headers, stream = assert(http_request.new_from_uri(url):go())
  local body = assert(stream:get_body_as_string())
  if headers:get ":status" ~= "200" then
    error(body)
  end
  local f = loadstring(body)
  return f()
end

while 1 do
  local client = server:accept()
  -- client:settimeout(10)
  local url, err = client:receive()
  if not err then client:send("Downloading "..url .. "\n") end
  run(url)
  client:close()
end
