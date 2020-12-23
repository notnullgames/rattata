local socket = require 'socket'

local server = assert(socket.bind("*", 0))

local ip, port = server:getsockname()

print("Please telnet to localhost on port " .. port .. ". Give it a URL to download & run.")

while 1 do
  local client = server:accept()
  -- client:settimeout(10)
  local line, err = client:receive()
  if not err then client:send(line .. "\n") end
  client:close()
end
