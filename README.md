JSON Any Types
=====


JSON Any Types


任意の形式のJSONをその形式のままRustに取り込む

```json
[{"x": 1, "y": "abcd", "z": {} , "a": []}]
```


実行
-----
```sh
# 単体テスト
cargo test

# jsonのパース
cargo run -- JSON_STRING
```


例
-----
```sh
> cargo run -- '[{"x": 1, "y": "abcd", "z": {} , "a": []}]'
input: [{"x": 1, "y": "abcd", "z": {} , "a": []}]
parse: Array([Object({"x": Int(1), "y": String("abcd"), "z": Object({}), "a": Array([])})])
json: [{"x":1,"y":"abcd","z":{},"a":[]}]

> cargo run -- '{aaa'
Error: Error("key must be a string", line: 1, column: 2)
```

