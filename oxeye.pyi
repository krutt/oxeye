"""
Type stubs for the oxeye module.

This module provides a Language Server Protocol (LSP) interface powered by PyO3 and Rust.
"""

def cmdline() -> None:
  """
  Start the LSP server directly.

  This method blocks and runs the LSP server, listening on stdin/stdout
  for LSP protocol messages.
  """
  ...

def get_capabilities() -> str:
  """
  Get the server capabilities as a JSON string.

  Returns:
      JSON string containing the LSP server capabilities
  """
  ...

def get_server_info() -> str:
  """
  Get information about the LSP server.

  Returns:
      Server information string including name and version
  """
  ...
