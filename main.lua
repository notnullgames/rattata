local socket = require 'socket'

local server = assert(socket.bind('*', 0))
local ip, port = server:getsockname()

print('Please telnet to localhost on port ' .. port .. '. Give it a URL to download & run.')


function run(url)
  print("You asked for "..url)
end

while 1 do
  local client = server:accept()
  local url, err = client:receive()
  if not err then client:send('Downloading '..url .. '\n') end
  run(url)
  client:close()
end
