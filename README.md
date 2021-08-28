# README

## Block structure:

```rust
pub struct Block {
   pub index: u32,
   pub timestamp: u128,
   pub hash: BlockHash,
   pub prev_block_hash: BlockHash,
   pub nonce: u64,
   pub payload: String,
}
```

## Hash

One small change in the input will change the hash drastically. This is called avalanching.

## Dependencies

**hex** // encoding and decoding data into/from hexadecimal representation.

**crypto-hash** // for cryptographic hash.

## Mining

Search randomly (brute-force) to fulfil the difficulty set by the network.

### Nonce

A nonce is arbitrary data. Anything really. The nonce is **the key** to generate a hash which matches the difficulty.

Setting the difficulty high enough protects against malicious users. Such a user must have more hashing power than the
entire network combined to 'be faster'.

### Steps

1. Generate new nonce
2. Hash bytes
3. Check hash against difficulty
   * on fail, go back to step 1
5. On success check, add to blockchain
6. Submit to peers (not included).

## Block verification

Bitcoin protocol has [19 verification steps](https://en.bitcoin.it/wiki/Protocol_rules#.22block.22_messages)

This project has 4 (to simplify/ for educational purposes). Which are:

1. Actual index == stored index value (Bitcoin doesn't store their index)
2. Block's hash fits stored difficulty value (INSECURE)
3. Time is always increasing (a real blockchain requires leniency here due to network latency/ sync)
4. Previous block hash == stored prev_block_hash (except genesis)

```rust
// TEST
// blockchain.blocks[3].index = 4;
// blockchain.blocks[3].hash[8] += 1;    //
// blockchain.blocks[3].payload = "Payload fail".to_owned();
// blockchain.blocks[3].prev_block_hash[18] = 8;
```
