# Details

Date : 2021-09-19 16:09:51

Directory c:\Documents\Rust\net_comms

Total : 49 files,  3305 codes, 531 comments, 880 blanks, all 4716 lines

[summary](results.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Cargo.toml](/Cargo.toml) | TOML | 22 | 1 | 10 | 33 |
| [library/Cargo.toml](/library/Cargo.toml) | TOML | 12 | 1 | 2 | 15 |
| [library/src/bytes/bytes.rs](/library/src/bytes/bytes.rs) | Rust | 85 | 28 | 33 | 146 |
| [library/src/bytes/from_bytes.rs](/library/src/bytes/from_bytes.rs) | Rust | 127 | 16 | 42 | 185 |
| [library/src/bytes/into_bytes.rs](/library/src/bytes/into_bytes.rs) | Rust | 40 | 3 | 16 | 59 |
| [library/src/bytes/mod.rs](/library/src/bytes/mod.rs) | Rust | 6 | 0 | 1 | 7 |
| [library/src/error/error.rs](/library/src/error/error.rs) | Rust | 328 | 36 | 14 | 378 |
| [library/src/error/error_kind.rs](/library/src/error/error_kind.rs) | Rust | 26 | 20 | 11 | 57 |
| [library/src/error/mod.rs](/library/src/error/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [library/src/lib.rs](/library/src/lib.rs) | Rust | 12 | 16 | 6 | 34 |
| [library/src/message/content_type.rs](/library/src/message/content_type.rs) | Rust | 17 | 16 | 4 | 37 |
| [library/src/message/into_message.rs](/library/src/message/into_message.rs) | Rust | 8 | 2 | 2 | 12 |
| [library/src/message/message.rs](/library/src/message/message.rs) | Rust | 236 | 81 | 62 | 379 |
| [library/src/message/metadata_type.rs](/library/src/message/metadata_type.rs) | Rust | 10 | 10 | 4 | 24 |
| [library/src/message/mod.rs](/library/src/message/mod.rs) | Rust | 8 | 0 | 2 | 10 |
| [library/src/packet/mod.rs](/library/src/packet/mod.rs) | Rust | 4 | 1 | 1 | 6 |
| [library/src/packet/packet.rs](/library/src/packet/packet.rs) | Rust | 158 | 69 | 52 | 279 |
| [library/src/packet/packet_kind.rs](/library/src/packet/packet_kind.rs) | Rust | 75 | 12 | 26 | 113 |
| [library/src/ron/from_ron.rs](/library/src/ron/from_ron.rs) | Rust | 15 | 7 | 3 | 25 |
| [library/src/ron/mod.rs](/library/src/ron/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [library/src/ron/to_ron.rs](/library/src/ron/to_ron.rs) | Rust | 32 | 11 | 8 | 51 |
| [library/tests/bytes.rs](/library/tests/bytes.rs) | Rust | 7 | 0 | 2 | 9 |
| [library/tests/main.rs](/library/tests/main.rs) | Rust | 4 | 0 | 2 | 6 |
| [library/tests/packet.rs](/library/tests/packet.rs) | Rust | 7 | 0 | 3 | 10 |
| [received_message.ron](/received_message.ron) | Rusty Object Notation (RON) | 29 | 0 | 0 | 29 |
| [src/bin/client/client.rs](/src/bin/client/client.rs) | Rust | 223 | 3 | 45 | 271 |
| [src/bin/client/client_config.ron](/src/bin/client/client_config.ron) | Rusty Object Notation (RON) | 6 | 0 | 0 | 6 |
| [src/bin/client/command/command.rs](/src/bin/client/command/command.rs) | Rust | 73 | 18 | 31 | 122 |
| [src/bin/client/command/command_raw.rs](/src/bin/client/command/command_raw.rs) | Rust | 235 | 66 | 50 | 351 |
| [src/bin/client/command/mod.rs](/src/bin/client/command/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [src/bin/client/main.rs](/src/bin/client/main.rs) | Rust | 26 | 3 | 11 | 40 |
| [src/bin/client/small_test.rs](/src/bin/client/small_test.rs) | Rust | 34 | 1 | 14 | 49 |
| [src/bin/server/database/mod.rs](/src/bin/server/database/mod.rs) | Rust | 20 | 0 | 10 | 30 |
| [src/bin/server/main.rs](/src/bin/server/main.rs) | Rust | 28 | 4 | 14 | 46 |
| [src/bin/server/message/command.rs](/src/bin/server/message/command.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/bin/server/message/mod.rs](/src/bin/server/message/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/bin/server/server.rs](/src/bin/server/server.rs) | Rust | 643 | 7 | 165 | 815 |
| [src/bin/server/server_config.ron](/src/bin/server/server_config.ron) | Rusty Object Notation (RON) | 6 | 0 | 0 | 6 |
| [src/bin/server/small_test.rs](/src/bin/server/small_test.rs) | Rust | 28 | 1 | 9 | 38 |
| [src/shared/config.rs](/src/shared/config.rs) | Rust | 6 | 8 | 2 | 16 |
| [src/shared/lib.rs](/src/shared/lib.rs) | Rust | 6 | 0 | 4 | 10 |
| [src/shared/message/content.rs](/src/shared/message/content.rs) | Rust | 101 | 0 | 33 | 134 |
| [src/shared/message/message_kind.rs](/src/shared/message/message_kind.rs) | Rust | 71 | 2 | 19 | 92 |
| [src/shared/message/metadata.rs](/src/shared/message/metadata.rs) | Rust | 242 | 62 | 74 | 378 |
| [src/shared/message/mod.rs](/src/shared/message/mod.rs) | Rust | 10 | 0 | 1 | 11 |
| [src/shared/message/request.rs](/src/shared/message/request.rs) | Rust | 47 | 10 | 22 | 79 |
| [src/shared/message/server_reply.rs](/src/shared/message/server_reply.rs) | Rust | 58 | 6 | 16 | 80 |
| [src/shared/user/mod.rs](/src/shared/user/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/shared/user/user.rs](/src/shared/user/user.rs) | Rust | 157 | 10 | 48 | 215 |

[summary](results.md)