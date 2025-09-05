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
      CompletionItem::new_simple("hello".to_string(), "A greeting".to_string()),
      CompletionItem::new_simple("world".to_string(), "The world".to_string()),
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
