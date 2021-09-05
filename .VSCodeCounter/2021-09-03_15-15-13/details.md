# Details

Date : 2021-09-03 15:15:13

Directory d:\stepa\Documents\Rust\net_comms

Total : 47 files,  2890 codes, 369 comments, 723 blanks, all 3982 lines

[summary](results.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Cargo.toml](/Cargo.toml) | TOML | 18 | 1 | 7 | 26 |
| [library/Cargo.toml](/library/Cargo.toml) | TOML | 12 | 1 | 2 | 15 |
| [library/src/bytes/bytes.rs](/library/src/bytes/bytes.rs) | Rust | 73 | 0 | 27 | 100 |
| [library/src/bytes/from_bytes.rs](/library/src/bytes/from_bytes.rs) | Rust | 135 | 7 | 39 | 181 |
| [library/src/bytes/into_bytes.rs](/library/src/bytes/into_bytes.rs) | Rust | 40 | 1 | 16 | 57 |
| [library/src/bytes/mod.rs](/library/src/bytes/mod.rs) | Rust | 6 | 0 | 1 | 7 |
| [library/src/error/error.rs](/library/src/error/error.rs) | Rust | 312 | 36 | 14 | 362 |
| [library/src/error/error_kind.rs](/library/src/error/error_kind.rs) | Rust | 25 | 19 | 11 | 55 |
| [library/src/error/mod.rs](/library/src/error/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [library/src/lib.rs](/library/src/lib.rs) | Rust | 11 | 17 | 8 | 36 |
| [library/src/message/content_type.rs](/library/src/message/content_type.rs) | Rust | 17 | 0 | 3 | 20 |
| [library/src/message/into_message.rs](/library/src/message/into_message.rs) | Rust | 8 | 0 | 1 | 9 |
| [library/src/message/message.rs](/library/src/message/message.rs) | Rust | 235 | 8 | 61 | 304 |
| [library/src/message/metadata_type.rs](/library/src/message/metadata_type.rs) | Rust | 10 | 0 | 2 | 12 |
| [library/src/message/mod.rs](/library/src/message/mod.rs) | Rust | 8 | 0 | 2 | 10 |
| [library/src/packet/mod.rs](/library/src/packet/mod.rs) | Rust | 4 | 1 | 1 | 6 |
| [library/src/packet/packet.rs](/library/src/packet/packet.rs) | Rust | 158 | 54 | 52 | 264 |
| [library/src/packet/packet_kind.rs](/library/src/packet/packet_kind.rs) | Rust | 75 | 11 | 26 | 112 |
| [library/src/ron/from_ron.rs](/library/src/ron/from_ron.rs) | Rust | 15 | 7 | 3 | 25 |
| [library/src/ron/into_ron.rs](/library/src/ron/into_ron.rs) | Rust | 32 | 11 | 8 | 51 |
| [library/src/ron/mod.rs](/library/src/ron/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [library/tests/bytes.rs](/library/tests/bytes.rs) | Rust | 10 | 0 | 2 | 12 |
| [library/tests/main.rs](/library/tests/main.rs) | Rust | 4 | 0 | 2 | 6 |
| [library/tests/packet.rs](/library/tests/packet.rs) | Rust | 7 | 0 | 3 | 10 |
| [src/bin/client/client.rs](/src/bin/client/client.rs) | Rust | 33 | 0 | 9 | 42 |
| [src/bin/client/client_config.ron](/src/bin/client/client_config.ron) | Rusty Object Notation (RON) | 3 | 0 | 0 | 3 |
| [src/bin/client/command/command.rs](/src/bin/client/command/command.rs) | Rust | 73 | 18 | 31 | 122 |
| [src/bin/client/command/command_raw.rs](/src/bin/client/command/command_raw.rs) | Rust | 235 | 66 | 50 | 351 |
| [src/bin/client/command/mod.rs](/src/bin/client/command/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [src/bin/client/main.rs](/src/bin/client/main.rs) | Rust | 111 | 6 | 21 | 138 |
| [src/bin/client/small_test.rs](/src/bin/client/small_test.rs) | Rust | 34 | 1 | 14 | 49 |
| [src/bin/server/main.rs](/src/bin/server/main.rs) | Rust | 28 | 2 | 11 | 41 |
| [src/bin/server/message/command.rs](/src/bin/server/message/command.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/bin/server/message/mod.rs](/src/bin/server/message/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/bin/server/server.rs](/src/bin/server/server.rs) | Rust | 462 | 3 | 78 | 543 |
| [src/bin/server/server_config.ron](/src/bin/server/server_config.ron) | Rusty Object Notation (RON) | 6 | 0 | 0 | 6 |
| [src/bin/server/small_test.rs](/src/bin/server/small_test.rs) | Rust | 28 | 1 | 9 | 38 |
| [src/shared/config.rs](/src/shared/config.rs) | Rust | 6 | 8 | 2 | 16 |
| [src/shared/lib.rs](/src/shared/lib.rs) | Rust | 5 | 0 | 2 | 7 |
| [src/shared/message/content.rs](/src/shared/message/content.rs) | Rust | 114 | 0 | 36 | 150 |
| [src/shared/message/message_kind.rs](/src/shared/message/message_kind.rs) | Rust | 71 | 2 | 19 | 92 |
| [src/shared/message/metadata.rs](/src/shared/message/metadata.rs) | Rust | 219 | 62 | 71 | 352 |
| [src/shared/message/mod.rs](/src/shared/message/mod.rs) | Rust | 10 | 0 | 1 | 11 |
| [src/shared/message/request.rs](/src/shared/message/request.rs) | Rust | 47 | 10 | 22 | 79 |
| [src/shared/message/server_reply.rs](/src/shared/message/server_reply.rs) | Rust | 58 | 6 | 16 | 80 |
| [src/shared/user/mod.rs](/src/shared/user/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/shared/user/user.rs](/src/shared/user/user.rs) | Rust | 115 | 10 | 34 | 159 |

[summary](results.md)