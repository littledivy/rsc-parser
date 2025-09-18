use rsc_parser::FlightResponse;

fn main() {
  let path = std::env::args().nth(1).expect("Please provide a file path");

  let chunks = std::fs::read_to_string(path).expect("Failed to read the file");
  let chunks = chunks.lines();

  let mut r = FlightResponse::new(true);
  for chunk in chunks {
    let mut row = chunk.to_string();
    if !row.ends_with('\n') {
      row.push('\n');
    }

    r.process_chunk(row);
  }

  println!("{:#?}", r);
}
