use serde_json::{Value as Json, json};

pub type RowParserState = u8; // 0..=4

const ROW_ID: RowParserState = 0;
const ROW_TAG: RowParserState = 1;
const ROW_CHUNK_BY_NEWLINE: RowParserState = 3;
const ROW_CHUNK_BY_LENGTH: RowParserState = 4;

#[derive(Debug, Clone)]
pub struct TextChunk {
  pub id: String,
  pub value: String,
  pub original_value: String,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct ModuleChunk {
  pub id: String,
  pub value: Json,
  pub original_value: String,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct ModelChunk {
  pub id: String,
  pub value: Json,
  pub original_value: String,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct HintChunk {
  pub id: String,
  pub code: String,
  pub value: Json,
  pub original_value: Json,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct ErrorDevChunk {
  pub id: String,
  pub error: Json,
  pub original_value: Json,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct ErrorProdChunk {
  pub id: String,
  pub error: Json,
  pub original_value: Json,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct PostponeDevChunk {
  pub id: String,
  pub error: Json,
  pub original_value: Json,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct PostponeProdChunk {
  pub id: String,
  pub error: Json,
  pub original_value: Json,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct BufferChunk {
  pub id: String,
  pub value: Vec<u8>,
  pub original_value: String,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct DebugInfoChunk {
  pub id: String,
  pub value: Json,
  pub original_value: String,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct ConsoleChunk {
  pub id: String,
  pub value: ConsoleValue,
  pub original_value: Json,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct ConsoleValue {
  pub method_name: String,
  pub stack_trace: Json,
  pub owner: Json,
  pub env: String,
  pub args: Vec<Json>,
}
#[derive(Debug, Clone)]
pub struct StartReadableStreamChunk {
  pub id: String,
  pub stream_type: Option<String>,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct StartAsyncIterableChunk {
  pub id: String,
  pub is_iterator: bool,
  pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct StopStreamChunk {
  pub id: String,
  pub final_model: String,
  pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum Chunk {
  Text(TextChunk),
  Module(ModuleChunk),
  Model(ModelChunk),
  Hint(HintChunk),
  ErrorDev(ErrorDevChunk),
  ErrorProd(ErrorProdChunk),
  PostponeDev(PostponeDevChunk),
  PostponeProd(PostponeProdChunk),
  Buffer(BufferChunk),
  DebugInfo(DebugInfoChunk),
  Console(ConsoleChunk),
  StartReadableStream(StartReadableStreamChunk),
  StartAsyncIterable(StartAsyncIterableChunk),
  StopStream(StopStreamChunk),
}

#[derive(Debug)]
pub struct FlightResponse {
  pub row_state: RowParserState,
  pub row_id: u64,
  pub row_tag: u8,
  pub row_length: usize,
  pub buffer: Vec<Vec<u8>>,
  pub chunks: Vec<Chunk>,
  pub current_timestamp: u64,
  pub dev: bool,
}

impl FlightResponse {
  pub fn new(dev: bool) -> Self {
    FlightResponse {
      buffer: vec![],
      row_id: 0,
      row_tag: 0,
      row_length: 0,
      row_state: ROW_ID,
      chunks: vec![],
      current_timestamp: 0,
      dev: dev,
    }
  }

  pub fn process_chunk(&mut self, chunk: impl AsRef<str>) {
    process_string_chunk(self, chunk.as_ref());
  }
}

fn hex_accumulate_u64(prev: u64, byte: u8) -> u64 {
  let v = if byte > b'`' {
    (byte - b'a') as u64 + 10
  } else {
    (byte - b'0') as u64
  };
  (prev << 4) | v
}

fn resolve_text(r: &mut FlightResponse, id: u64, text: &str) {
  r.chunks.push(Chunk::Text(TextChunk {
    id: format!("{:x}", id),
    value: text.into(),
    original_value: text.into(),
    timestamp: r.current_timestamp,
  }));
}
fn resolve_model(r: &mut FlightResponse, id: u64, model: &str) {
  let parsed_value =
    serde_json::from_str(model).unwrap_or_else(|_| json!(model));
  r.chunks.push(Chunk::Model(ModelChunk {
    id: format!("{:x}", id),
    value: parsed_value,
    original_value: model.into(),
    timestamp: r.current_timestamp,
  }));
}
fn resolve_module(r: &mut FlightResponse, id: u64, model: &str) {
  let parsed_value =
    serde_json::from_str(model).unwrap_or_else(|_| json!(model));
  r.chunks.push(Chunk::Module(ModuleChunk {
    id: format!("{:x}", id),
    value: parsed_value,
    original_value: model.into(),
    timestamp: r.current_timestamp,
  }));
}

fn process_full_string_row(
  r: &mut FlightResponse,
  id: u64,
  tag: u8,
  row: &str,
) {
  match tag {
    b'T' => resolve_text(r, id, row),
    b'I' => resolve_module(r, id, row),
    _ => resolve_model(r, id, row),
  }
}

fn process_string_chunk(response: &mut FlightResponse, chunk: &str) {
  let mut i = 0;
  let chars: Vec<char> = chunk.chars().collect();
  let mut row_state = response.row_state;
  let mut row_id = response.row_id;
  let mut row_tag = response.row_tag;
  let mut row_length = response.row_length;
  let mut buffer = response.buffer.clone();
  while i < chars.len() {
    let mut last_idx = -1isize;
    match row_state {
      ROW_ID => {
        let b = chars[i] as u8;
        i += 1;
        if b == b':' {
          row_state = ROW_TAG
        } else {
          row_id = hex_accumulate_u64(row_id, b)
        };
        continue;
      }
      ROW_TAG => {
        let ch = chars[i] as u8;
        // Check if this looks like a tag (single letter like I, T, etc) followed by content
        // If not, treat it as the start of content with no explicit tag
        if i + 1 < chars.len()
          && (ch == b'I' || ch == b'T')
          && chars[i + 1] != ':'
        {
          row_tag = ch;
          i += 1;
        } else {
          // No explicit tag, content starts here
          row_tag = 0;
        }
        row_state = ROW_CHUNK_BY_NEWLINE;
        continue;
      }
      ROW_CHUNK_BY_NEWLINE => {
        for (j, c) in chars[i..].iter().enumerate() {
          if *c == '\n' {
            last_idx = (i + j) as isize;
            break;
          }
        }
      }
      ROW_CHUNK_BY_LENGTH => last_idx = chars.len() as isize,
      _ => {}
    }
    if last_idx > -1 {
      let li = last_idx as usize;
      let row: String = chars[i..li].iter().collect();
      process_full_string_row(response, row_id, row_tag, &row);
      i = li + 1;
      row_state = ROW_ID;
      row_tag = 0;
      row_id = 0;
      row_length = 0;
      buffer.clear()
    } else if chunk.len() != i {
      panic!("incomplete row: {}", &chunk[i..]);
    }
  }
  response.row_state = row_state;
  response.row_id = row_id;
  response.row_tag = row_tag;
  response.row_length = row_length;
  response.buffer = buffer;
}
