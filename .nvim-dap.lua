local dap = require("dap")

dap.adapters.lldb = {
  type = "executable",
  command = "/usr/bin/lldb-dap", -- adjust as needed
  name = "lldb",
}

dap.configurations.rust = {
  {
    name = "blpapi-rs",
    type = "lldb",
    request = "launch",
    program = function()
            return vim.fn.getcwd() .. "/target/debug/blpapi-rs"
    end,
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
  },
}
