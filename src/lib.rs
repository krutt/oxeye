/* ~~/src/lib.rs */

use pyo3::prelude::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Documentation module with compile-time included markdown files
mod docs {
  // Keywords documentation
  pub const CONST: &str = include_str!("../static/docs/keywords/const.md");
  pub const FN: &str = include_str!("../static/docs/keywords/fn.md");
  pub const LET: &str = include_str!("../static/docs/keywords/let.md");
  pub const MOD: &str = include_str!("../static/docs/keywords/mod.md");
  pub const ASSERT: &str = include_str!("../static/docs/keywords/assert.md");
  pub const MATCH: &str = include_str!("../static/docs/keywords/match.md");
  
  // Types documentation  
  pub const U256: &str = include_str!("../static/docs/types/u256.md");
  pub const U128: &str = include_str!("../static/docs/types/u128.md");
  pub const U64: &str = include_str!("../static/docs/types/u64.md");
  pub const U32: &str = include_str!("../static/docs/types/u32.md");
  pub const U16: &str = include_str!("../static/docs/types/u16.md");
  pub const U8: &str = include_str!("../static/docs/types/u8.md");
  pub const PUBKEY: &str = include_str!("../static/docs/types/pubkey.md");
  pub const SIGNATURE: &str = include_str!("../static/docs/types/signature.md");
  pub const HEIGHT: &str = include_str!("../static/docs/types/height.md");
  pub const CTX8: &str = include_str!("../static/docs/types/ctx8.md");
  pub const EITHER: &str = include_str!("../static/docs/types/either.md");
  pub const OPTION: &str = include_str!("../static/docs/types/option.md");
  pub const LEFT: &str = include_str!("../static/docs/types/left.md");
  pub const RIGHT: &str = include_str!("../static/docs/types/right.md");
  pub const SOME: &str = include_str!("../static/docs/types/some.md");
  pub const NONE: &str = include_str!("../static/docs/types/none.md");
  
  // Jet functions documentation
  pub const JET_BIP_0340_VERIFY: &str = include_str!("../static/docs/jet_functions/bip_0340_verify.md");
  pub const JET_EQ_256: &str = include_str!("../static/docs/jet_functions/eq_256.md");
  pub const JET_SHA_256_CTX_8_INIT: &str = include_str!("../static/docs/jet_functions/sha_256_ctx_8_init.md");
  pub const JET_SHA_256_CTX_8_ADD_32: &str = include_str!("../static/docs/jet_functions/sha_256_ctx_8_add_32.md");
  pub const JET_SHA_256_CTX_8_FINALIZE: &str = include_str!("../static/docs/jet_functions/sha_256_ctx_8_finalize.md");
  pub const JET_CHECK_LOCK_HEIGHT: &str = include_str!("../static/docs/jet_functions/check_lock_height.md");
  pub const JET_SIG_ALL_HASH: &str = include_str!("../static/docs/jet_functions/sig_all_hash.md");
  pub const JET_ADD_256: &str = include_str!("../static/docs/jet_functions/add_256.md");
  pub const JET_SUBTRACT_256: &str = include_str!("../static/docs/jet_functions/subtract_256.md");
  pub const JET_AND_256: &str = include_str!("../static/docs/jet_functions/and_256.md");
  pub const JET_OR_256: &str = include_str!("../static/docs/jet_functions/or_256.md");
  
  // Miscellaneous
  pub const HEX_PREFIX: &str = include_str!("../static/docs/misc/hex_prefix.md");
  pub const WITNESS_ACCESS: &str = include_str!("../static/docs/misc/witness_access.md");
  pub const PARAM_ACCESS: &str = include_str!("../static/docs/misc/param_access.md");
  
  // Comprehensive reference
  pub const COMPREHENSIVE: &str = include_str!("../static/docs/comprehensive/reference.md");
  
  // Helper function to get documentation by symbol
  pub fn get_doc(symbol: &str) -> Option<&'static str> {
    match symbol {
      // Keywords
      "const" => Some(CONST),
      "fn" => Some(FN),
      "let" => Some(LET),
      "mod" => Some(MOD),
      "assert!" => Some(ASSERT),
      "match" => Some(MATCH),
      
      // Types
      "u256" => Some(U256),
      "u128" => Some(U128),
      "u64" => Some(U64),
      "u32" => Some(U32),
      "u16" => Some(U16),
      "u8" => Some(U8),
      "Pubkey" => Some(PUBKEY),
      "Signature" => Some(SIGNATURE),
      "Height" => Some(HEIGHT),
      "Ctx8" => Some(CTX8),
      "Either" => Some(EITHER),
      "Option" => Some(OPTION),
      "Left" => Some(LEFT),
      "Right" => Some(RIGHT),
      "Some" => Some(SOME),
      "None" => Some(NONE),
      
      // Jet functions
      "jet::bip_0340_verify" => Some(JET_BIP_0340_VERIFY),
      "jet::eq_256" => Some(JET_EQ_256),
      "jet::sha_256_ctx_8_init" => Some(JET_SHA_256_CTX_8_INIT),
      "jet::sha_256_ctx_8_add_32" => Some(JET_SHA_256_CTX_8_ADD_32),
      "jet::sha_256_ctx_8_finalize" => Some(JET_SHA_256_CTX_8_FINALIZE),
      "jet::check_lock_height" => Some(JET_CHECK_LOCK_HEIGHT),
      "jet::sig_all_hash" => Some(JET_SIG_ALL_HASH),
      "jet::add_256" => Some(JET_ADD_256),
      "jet::subtract_256" => Some(JET_SUBTRACT_256),
      "jet::and_256" => Some(JET_AND_256),
      "jet::or_256" => Some(JET_OR_256),
      
      // Miscellaneous
      "0x" => Some(HEX_PREFIX),
      "witness::" => Some(WITNESS_ACCESS),
      "param::" => Some(PARAM_ACCESS),
      
      _ => None,
    }
  }
  
  pub fn get_comprehensive() -> &'static str {
    COMPREHENSIVE
  }
}

#[derive(Debug)]
struct Backend {
  client: Client,
  documents: Arc<RwLock<HashMap<Url, String>>>,
}

impl Backend {
  async fn get_hover_information(&self, uri: &tower_lsp::lsp_types::Url, position: Position) -> Option<String> {
    // Get the document content
    let documents = self.documents.read().await;
    let content = documents.get(uri)?;
    
    // Extract the symbol at the cursor position
    let symbol = self.extract_symbol_at_position(content, position)?;
    
    // First, try to find a function definition in the document
    if let Some(function_info) = self.extract_function_definition(content, &symbol) {
      return Some(function_info);
    }
    
    // Try to find a constant definition in the document
    if let Some(const_info) = self.extract_constant_definition(content, &symbol) {
      return Some(const_info);
    }
    
    // Fall back to built-in symbol information
    Some(self.get_specific_hover_info(&symbol))
  }
  
  fn extract_symbol_at_position(&self, content: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let line_index = position.line as usize;
    let char_index = position.character as usize;
    
    // Check if position is valid
    if line_index >= lines.len() {
      return None;
    }
    
    let line = lines[line_index];
    
    // Convert line to bytes for proper indexing
    let line_bytes = line.as_bytes();
    if char_index >= line_bytes.len() {
      return None;
    }
    
    // Find word boundaries around the cursor position
    let chars: Vec<char> = line.chars().collect();
    let char_positions: Vec<usize> = line.char_indices().map(|(i, _)| i).collect();
    
    // Find the character index in the chars array
    let mut char_idx = 0;
    for (i, &byte_pos) in char_positions.iter().enumerate() {
      if byte_pos <= char_index {
        char_idx = i;
      } else {
        break;
      }
    }
    
    if char_idx >= chars.len() {
      return None;
    }
    
    // Check if we're on a valid symbol character or special characters
    let current_char = chars[char_idx];
    if !current_char.is_alphanumeric() && current_char != '_' && current_char != ':' && current_char != '!' && current_char != 'x' {
      return None;
    }
    
    // Handle special case for "0x" prefix
    if current_char == 'x' && char_idx > 0 && chars[char_idx - 1] == '0' {
      return Some("0x".to_string());
    }
    
    // Find the start of the symbol
    let mut start = char_idx;
    while start > 0 {
      let ch = chars[start - 1];
      if ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == '!' {
        start -= 1;
      } else {
        break;
      }
    }
    
    // Find the end of the symbol
    let mut end = char_idx + 1;
    while end < chars.len() {
      let ch = chars[end];
      if ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == '!' {
        end += 1;
      } else {
        break;
      }
    }
    
    // Extract the symbol
    let symbol: String = chars[start..end].iter().collect();
    
    // Handle special cases and normalize symbols
    if symbol.starts_with("jet::") {
      Some(symbol)
    } else if symbol == "assert!" {
      Some("assert!".to_string())
    } else if symbol == "0x" {
      Some("0x".to_string())
    } else if symbol.contains("::") {
      // Handle module access patterns like "witness::" or "param::"
      if symbol.ends_with("::") {
        Some(symbol)
      } else {
        // Extract just the part after "::" for nested symbols
        if let Some(last_part) = symbol.split("::").last() {
          if !last_part.is_empty() {
            Some(last_part.to_string())
          } else {
            Some(symbol)
          }
        } else {
          Some(symbol)
        }
      }
    } else if !symbol.is_empty() {
      Some(symbol)
    } else {
      None
    }
  }
  
  fn extract_function_definition(&self, content: &str, function_name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    
    // Look for function definition
    for (i, line) in lines.iter().enumerate() {
      let trimmed = line.trim();
      
      // Check if this line contains a function definition for our symbol
      if trimmed.starts_with("fn ") {
        // Extract the function name from the definition
        if let Some(fn_start) = trimmed.find("fn ") {
          let after_fn = &trimmed[fn_start + 3..];
          if let Some(paren_pos) = after_fn.find('(') {
            let extracted_name = after_fn[..paren_pos].trim();
            if extracted_name == function_name {
              // Extract the function signature
              let mut signature = String::new();
              let mut j = i;
              
              // Collect the complete function signature (might span multiple lines)
              while j < lines.len() {
                let current_line = lines[j].trim();
                signature.push_str(current_line);
                
                // If we find the opening brace, we have the complete signature
                if current_line.contains("{") {
                  break;
                }
                signature.push(' ');
                j += 1;
              }
              
              // Extract function body for context
              let body_start = j + 1;
              let mut body_lines = Vec::new();
              let mut brace_count = 1;
              let _body_end = body_start;
              
              for k in body_start..lines.len() {
                let line = lines[k];
                for ch in line.chars() {
                  match ch {
                    '{' => brace_count += 1,
                    '}' => {
                      brace_count -= 1;
                      if brace_count == 0 {
                        break;
                      }
                    },
                    _ => {}
                  }
                }
                if brace_count == 0 {
                  break;
                }
                body_lines.push(line);
              }
              
              // Look for comments above the function (docstring)
              let mut doc_lines = Vec::new();
              if i > 0 {
                let mut doc_i = i - 1;
                loop {
                  let line = lines[doc_i].trim();
                  if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
                    doc_lines.insert(0, line);
                    if doc_i == 0 {
                      break;
                    }
                    doc_i -= 1;
                  } else if line.is_empty() && doc_i > 0 {
                    // Skip empty lines between comments and function
                    doc_i -= 1;
                  } else {
                    break;
                  }
                }
              }
              
              // Format the hover information
              let mut hover_content = format!("# `{}` - User-Defined Function\n\n", function_name);
              
              // Add documentation if found
              if !doc_lines.is_empty() {
                hover_content.push_str("## Documentation\n");
                for doc_line in &doc_lines {
                  let cleaned = doc_line.trim_start_matches("//").trim_start_matches("/*").trim_start_matches("*").trim();
                  if !cleaned.is_empty() {
                    hover_content.push_str(&format!("{}\n", cleaned));
                  }
                }
                hover_content.push('\n');
              }
              
              // Add function signature
              hover_content.push_str("## Signature\n```simplicity\n");
              hover_content.push_str(&signature);
              hover_content.push_str("\n```\n\n");
              
              // Add function body (first few lines for context)
              if !body_lines.is_empty() {
                hover_content.push_str("## Implementation\n```simplicity\n");
                let preview_lines = body_lines.iter().take(10).collect::<Vec<_>>();
                for line in preview_lines {
                  hover_content.push_str(line);
                  hover_content.push('\n');
                }
                if body_lines.len() > 10 {
                  hover_content.push_str("  // ... rest of function\n");
                }
                hover_content.push_str("```\n");
              }
              
              return Some(hover_content);
            }
          }
        }
      }
    }
    
    None
  }
  
  fn extract_constant_definition(&self, content: &str, constant_name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    
    // Look for constant definition
    for (i, line) in lines.iter().enumerate() {
      let trimmed = line.trim();
      
      // Check if this line contains a constant definition for our symbol
      if trimmed.starts_with("const ") && trimmed.contains(constant_name) {
        // Extract the constant name from the definition
        if let Some(const_start) = trimmed.find("const ") {
          let after_const = &trimmed[const_start + 6..];
          if let Some(colon_pos) = after_const.find(':') {
            let extracted_name = after_const[..colon_pos].trim();
            if extracted_name == constant_name {
              // Look for comments above the constant (docstring)
              let mut doc_lines = Vec::new();
              if i > 0 {
                let mut doc_i = i - 1;
                loop {
                  let line = lines[doc_i].trim();
                  if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
                    doc_lines.insert(0, line);
                    if doc_i == 0 {
                      break;
                    }
                    doc_i -= 1;
                  } else if line.is_empty() && doc_i > 0 {
                    // Skip empty lines between comments and constant
                    doc_i -= 1;
                  } else {
                    break;
                  }
                }
              }
              
              // Format the hover information
              let mut hover_content = format!("# `{}` - User-Defined Constant\n\n", constant_name);
              
              // Add documentation if found
              if !doc_lines.is_empty() {
                hover_content.push_str("## Documentation\n");
                for doc_line in &doc_lines {
                  let cleaned = doc_line.trim_start_matches("//").trim_start_matches("/*").trim_start_matches("*").trim();
                  if !cleaned.is_empty() {
                    hover_content.push_str(&format!("{}\n", cleaned));
                  }
                }
                hover_content.push('\n');
              }
              
              // Add constant definition
              hover_content.push_str("## Definition\n```simplicity\n");
              hover_content.push_str(trimmed);
              hover_content.push_str("\n```\n");
              
              return Some(hover_content);
            }
          }
        }
      }
    }
    
    None
  }
  
  fn get_specific_hover_info(&self, symbol: &str) -> String {
    // First try to get documentation from external files
    if let Some(doc_content) = docs::get_doc(symbol) {
      return doc_content.to_string();
    }
    
    // Default case - return comprehensive reference for unknown symbols
    docs::get_comprehensive().to_string()
  }
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
    let mut documents = self.documents.write().await;
    documents.insert(
      params.text_document.uri.clone(),
      params.text_document.text.clone(),
    );
    
    self
      .client
      .log_message(
        MessageType::INFO,
        format!("File opened: {}", params.text_document.uri),
      )
      .await;
  }

  async fn did_change(&self, params: DidChangeTextDocumentParams) {
    if let Some(change) = params.content_changes.first() {
      let mut documents = self.documents.write().await;
      documents.insert(
        params.text_document.uri.clone(),
        change.text.clone(),
      );
    }
    
    self
      .client
      .log_message(
        MessageType::INFO,
        format!("File changed: {}", params.text_document.uri),
      )
      .await;
  }

  async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
    let position = params.text_document_position_params.position;
    let uri = params.text_document_position_params.text_document.uri;    
    let hover_info = self.get_hover_information(&uri, position).await;
    match hover_info {
      Some(info) => Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
          kind: MarkupKind::Markdown,
          value: info,
        }),
        range: None,
      })),
      None => Ok(None),
    }
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
      let (service, socket) = LspService::new(|client| Backend { 
        client,
        documents: Arc::new(RwLock::new(HashMap::new())),
      });
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
