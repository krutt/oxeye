/* ~~/src/lib.rs */

use pyo3::prelude::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
  client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
  async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
    Ok(InitializeResult {
      capabilities: ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions {
          resolve_provider: Some(false),
          trigger_characters: Some(vec![".".to_string()]),
          work_done_progress_options: Default::default(),
          all_commit_characters: None,
          completion_item: None,
        }),
        ..ServerCapabilities::default()
      },
      server_info: Some(ServerInfo {
        name: "Oxeye Language Server".to_string(),
        version: Some("0.1.0".to_string()),
      }),
    })
  }

  async fn initialized(&self, _: InitializedParams) {
    self
      .client
      .log_message(MessageType::INFO, "Oxeye Language Server initialized!")
      .await;
  }

  async fn shutdown(&self) -> Result<()> {
    Ok(())
  }

  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    self
      .client
      .log_message(
        MessageType::INFO,
        format!("File opened: {}", params.text_document.uri),
      )
      .await;
  }

  async fn did_change(&self, params: DidChangeTextDocumentParams) {
    self
      .client
      .log_message(
        MessageType::INFO,
        format!("File changed: {}", params.text_document.uri),
      )
      .await;
  }

  async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
    Ok(Some(Hover {
      contents: HoverContents::Scalar(MarkedString::String(
        "Oxeye Language Server - Hover information".to_string(),
      )),
      range: None,
    }))
  }

  async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
    Ok(Some(CompletionResponse::Array(vec![
      // Keywords
      CompletionItem::new_simple("const".to_string(), "variable keyword".to_string()),
      CompletionItem::new_simple("fn".to_string(), "function keyword".to_string()),
      CompletionItem::new_simple("let".to_string(), "variable keyword".to_string()),
      CompletionItem::new_simple("mod".to_string(), "module keyword".to_string()),
      CompletionItem::new_simple("match".to_string(), "pattern matching keyword".to_string()),
      CompletionItem::new_simple("assert!".to_string(), "assertion macro".to_string()),
      CompletionItem::new_simple("main".to_string(), "main function".to_string()),
      
      // Control flow
      CompletionItem::new_simple("Left".to_string(), "Either left variant".to_string()),
      CompletionItem::new_simple("Right".to_string(), "Either right variant".to_string()),
      CompletionItem::new_simple("Some".to_string(), "Option some variant".to_string()),
      CompletionItem::new_simple("None".to_string(), "Option none variant".to_string()),
      
      // Types
      CompletionItem::new_simple("u256".to_string(), "256-bit unsigned integer type".to_string()),
      CompletionItem::new_simple("u128".to_string(), "128-bit unsigned integer type".to_string()),
      CompletionItem::new_simple("u64".to_string(), "64-bit unsigned integer type".to_string()),
      CompletionItem::new_simple("u32".to_string(), "32-bit unsigned integer type".to_string()),
      CompletionItem::new_simple("u16".to_string(), "16-bit unsigned integer type".to_string()),
      CompletionItem::new_simple("u8".to_string(), "8-bit unsigned integer type".to_string()),
      CompletionItem::new_simple("Either".to_string(), "Either type for sum types".to_string()),
      CompletionItem::new_simple("Option".to_string(), "Option type for nullable values".to_string()),
      CompletionItem::new_simple("Pubkey".to_string(), "Bitcoin public key type".to_string()),
      CompletionItem::new_simple("Signature".to_string(), "Digital signature type".to_string()),
      CompletionItem::new_simple("Ctx8".to_string(), "SHA-256 context type".to_string()),
      CompletionItem::new_simple("Height".to_string(), "Block height type".to_string()),
      
      // Jet functions - Cryptographic
      CompletionItem::new_simple("jet::sha_256_ctx_8_init".to_string(), "Initialize SHA-256 context".to_string()),
      CompletionItem::new_simple("jet::sha_256_ctx_8_add_32".to_string(), "Add 32 bytes to SHA-256 context".to_string()),
      CompletionItem::new_simple("jet::sha_256_ctx_8_finalize".to_string(), "Finalize SHA-256 hash".to_string()),
      CompletionItem::new_simple("jet::bip_0340_verify".to_string(), "Verify BIP-340 Schnorr signature".to_string()),
      CompletionItem::new_simple("jet::sig_all_hash".to_string(), "Get signature hash for all inputs".to_string()),
      
      // Jet functions - Comparison
      CompletionItem::new_simple("jet::eq_256".to_string(), "Compare two 256-bit values for equality".to_string()),
      CompletionItem::new_simple("jet::eq_128".to_string(), "Compare two 128-bit values for equality".to_string()),
      CompletionItem::new_simple("jet::eq_64".to_string(), "Compare two 64-bit values for equality".to_string()),
      CompletionItem::new_simple("jet::eq_32".to_string(), "Compare two 32-bit values for equality".to_string()),
      CompletionItem::new_simple("jet::eq_16".to_string(), "Compare two 16-bit values for equality".to_string()),
      CompletionItem::new_simple("jet::eq_8".to_string(), "Compare two 8-bit values for equality".to_string()),
      
      // Jet functions - Bitcoin
      CompletionItem::new_simple("jet::check_lock_height".to_string(), "Check if height lock is satisfied".to_string()),
      CompletionItem::new_simple("jet::check_lock_time".to_string(), "Check if time lock is satisfied".to_string()),
      CompletionItem::new_simple("jet::current_index".to_string(), "Get current input index".to_string()),
      CompletionItem::new_simple("jet::current_amount".to_string(), "Get current input amount".to_string()),
      CompletionItem::new_simple("jet::current_asset".to_string(), "Get current input asset".to_string()),
      CompletionItem::new_simple("jet::current_script_hash".to_string(), "Get current script hash".to_string()),
      
      // Jet functions - Arithmetic
      CompletionItem::new_simple("jet::add_256".to_string(), "Add two 256-bit values".to_string()),
      CompletionItem::new_simple("jet::add_128".to_string(), "Add two 128-bit values".to_string()),
      CompletionItem::new_simple("jet::add_64".to_string(), "Add two 64-bit values".to_string()),
      CompletionItem::new_simple("jet::add_32".to_string(), "Add two 32-bit values".to_string()),
      CompletionItem::new_simple("jet::add_16".to_string(), "Add two 16-bit values".to_string()),
      CompletionItem::new_simple("jet::add_8".to_string(), "Add two 8-bit values".to_string()),
      CompletionItem::new_simple("jet::subtract_256".to_string(), "Subtract two 256-bit values".to_string()),
      CompletionItem::new_simple("jet::subtract_128".to_string(), "Subtract two 128-bit values".to_string()),
      CompletionItem::new_simple("jet::subtract_64".to_string(), "Subtract two 64-bit values".to_string()),
      CompletionItem::new_simple("jet::subtract_32".to_string(), "Subtract two 32-bit values".to_string()),
      CompletionItem::new_simple("jet::subtract_16".to_string(), "Subtract two 16-bit values".to_string()),
      CompletionItem::new_simple("jet::subtract_8".to_string(), "Subtract two 8-bit values".to_string()),
      
      // Jet functions - Bitwise
      CompletionItem::new_simple("jet::and_256".to_string(), "Bitwise AND on 256-bit values".to_string()),
      CompletionItem::new_simple("jet::or_256".to_string(), "Bitwise OR on 256-bit values".to_string()),
      CompletionItem::new_simple("jet::xor_256".to_string(), "Bitwise XOR on 256-bit values".to_string()),
      CompletionItem::new_simple("jet::not_256".to_string(), "Bitwise NOT on 256-bit value".to_string()),
      CompletionItem::new_simple("jet::shift_left_256".to_string(), "Left shift 256-bit value".to_string()),
      CompletionItem::new_simple("jet::shift_right_256".to_string(), "Right shift 256-bit value".to_string()),
      
      // Common patterns and snippets
      CompletionItem::new_simple("witness::".to_string(), "Access witness module".to_string()),
      CompletionItem::new_simple("param::".to_string(), "Access parameter module".to_string()),
      CompletionItem::new_simple("0x".to_string(), "Hexadecimal number prefix".to_string()),
      
      // Array types
      CompletionItem::new_simple("[u8; 32]".to_string(), "32-byte array type".to_string()),
      CompletionItem::new_simple("[u8; 64]".to_string(), "64-byte array type".to_string()),
      CompletionItem::new_simple("[u8; 16]".to_string(), "16-byte array type".to_string()),
      
      // Common variable names in blockchain context
      CompletionItem::new_simple("pubkey".to_string(), "Public key variable".to_string()),
      CompletionItem::new_simple("signature".to_string(), "Signature variable".to_string()),
      CompletionItem::new_simple("hash".to_string(), "Hash variable".to_string()),
      CompletionItem::new_simple("preimage".to_string(), "Hash preimage variable".to_string()),
      CompletionItem::new_simple("timeout".to_string(), "Timeout variable".to_string()),
      CompletionItem::new_simple("amount".to_string(), "Amount variable".to_string()),
      
      // Function patterns
      CompletionItem::new_simple("-> u256".to_string(), "Return type: 256-bit unsigned integer".to_string()),
      CompletionItem::new_simple("-> bool".to_string(), "Return type: boolean".to_string()),
      CompletionItem::new_simple("-> ()".to_string(), "Return type: unit (void)".to_string()),
    ])))
  }
}

/// Get server capabilities as JSON string
#[pyfunction]
fn get_capabilities() -> PyResult<String> {
  let capabilities = ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
    completion_provider: Some(CompletionOptions {
      resolve_provider: Some(false),
      trigger_characters: Some(vec![".".to_string()]),
      work_done_progress_options: Default::default(),
      all_commit_characters: None,
      completion_item: None,
    }),
    ..ServerCapabilities::default()
  };

  serde_json::to_string_pretty(&capabilities).map_err(|e| {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Serialization error: {}", e))
  })
}


#[pyfunction]
fn get_server_info() -> PyResult<String> {
  Ok("Oxeye Language Server v0.1.0 - A language server powered by PyO3 and Rust".to_string())
}


#[pyfunction]
fn serve(py: Python) -> PyResult<()> {
  py.allow_threads(|| {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
      let stdin = tokio::io::stdin();
      let stdout = tokio::io::stdout();
      let (service, socket) = LspService::new(|client| Backend { client });
      Server::new(stdin, stdout, socket).serve(service).await;
    });
  });
  Ok(())
}


/// Oxeye Language Server Protocol server for Simplicity blockchain programming language
#[pymodule]
fn oxeye(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(get_capabilities, m)?)?;
  m.add_function(wrap_pyfunction!(get_server_info, m)?)?;
  m.add_function(wrap_pyfunction!(serve, m)?)?;
  Ok(())
}
