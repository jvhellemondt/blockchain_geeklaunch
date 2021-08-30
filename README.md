# Proof of work Blockchain in Rust

[Youtube series GeekLaunch](https://www.youtube.com/playlist?list=PLwnSaD6BDfXL0RiKT_5nOIdxTxZWpPtAv)

This project is not a secure blockchain (nor working one). Some things which are still missing:

* The difficulty stored in a block is not validated
* The value of the coinbase transaction is not validated
* "Coin ownership" is neither enforced nor existent
* Identical outputs from different transactions are indistinguishable
* ... more...

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

## Transactions

Do remember, a blockchain is not a spreadsheet.

We have to protect against:

* Overspending (where did the money come from?)
* Double-spending (is the money available?)
* Impersonation (who owns the money and who is sending it? -> asymmetric cryptography: mathematically verify the person)
* ... there are more. See [Bitcoin's](https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages)

### Structure of a Transaction

There are two components: inputs and outputs. _And inputs are outputs_.....

Transactions only contain two important pieces of information:

* Set of inputs (which are unused outputs from previous transactions)
* Set of outputs (_new_ outputs that can be used in future transactions)

With this information we can calculate...

* ... the value of the transaction: Σinputs
* ... the value of the fee: Σinputs - Σoutputs

#### Fee

A fee should be present as an incentive for the miner to add the transaction to their block.

### Coinbase transaction

A coinbase transaction is the first transaction in a block. It is a unique type of bitcoin transaction that can be
created by a miner. The miners use it to collect the block reward for their work and any other transaction fees
collected by the miner are also sent in this transaction.

## Rust specific

### Errors

Result<T,E> is a recoverable error panic! is a program crash error

### Null

Null is not included in Rust. Rust implemented Option<T>. Option comes in two flavors:

* Option::Some(T)
* Option::None // like null
