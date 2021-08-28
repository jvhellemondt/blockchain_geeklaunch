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

## Mining strategy

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
