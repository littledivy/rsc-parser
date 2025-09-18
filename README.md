## rsc-parser

Rust crate for parsing RSC (React server components) payloads.

```
$ cargo run -- ./payload

FlightResponse {
    row_state: 0,
    row_id: 0,
    row_tag: 0,
    row_length: 0,
    buffer: [],
    chunks: [
        Model(
            ModelChunk {
                id: "0",
                value: Array [
                    String("development"),
                    Array [
                        Array [
                            String("children"),
                            String("(main)"),
                            String("children"),
                            String("__PAGE__"),
                            Array [
                                String("__PAGE__"),
                                Object {},
                            ],
                            String("$L1"),
                            Array [
                                Null,
                                String("$L2"),
                            ],
                        ],
                    ],
                ],
                original_value: "[\"development\",[[\"children\",\"(main)\",\"children\",\"__PAGE__\",[\"__PAGE__\",{}],\"$L1\",[null,\"$L2\"]]]]",
                timestamp: 0,
            },
        ),
        Module(
            ModuleChunk {
                id: "4",
                value: Array [
                    String("(app-pages-browser)/./app/(main)/components/Pronunciation.tsx"),
                    Array [
                        String("app/(main)/page"),
                        String("static/chunks/app/(main)/page.js"),
                    ],
                    String("Pronunciation"),
                ],
                original_value: "[\"(app-pages-browser)/./app/(main)/components/Pronunciation.tsx\",[\"app/(main)/page\",\"static/chunks/app/(main)/page.js\"],\"Pronunciation\"]",
                timestamp: 0,
            },
        ),
        // ...
```
