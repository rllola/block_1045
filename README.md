# Block 1045 bug

Here a minimal exemple of the JSON parsing bug in Namada.

## Description

When querying for a block throught the tendermint rpc endpoint it returns a block encoded in JSON. The data might include evidences. In the case of block 1045 on Namada testnet it does. The block can be find under `./block.json`. However when attempting to read it using the `namada` crate v0.21.1, it fails.

The block can be retrieve using curl.
```bash
$ curl localhost:26657/block?height=1045 
```

## Reproducing the error

```bash
$ make install-deps
$ make run
```

The error 
```
    Finished dev [unoptimized + debuginfo] target(s) in 1m 25s
     Running `target/debug/block_1045`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("invalid type: integer `0`, expected a string", line: 74, column: 34)', src/main.rs:9:52
```

The line 74 in the json:
```json
73                        "height": "1044",
74                        "round": 0,
75                        "block_id": {
```

We can see that the `round` field is an integer and not a string. 