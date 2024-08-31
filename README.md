<h1 align="center">
🌬️🎙️ seed_entropy_blower 🎲🗝
<br><br>
<img src="seed_entropy_blower.png" alt="SRI" width="200">
</h1>

<h3 align="center">
blow into your mic to generate some entropy.

use that entropy to create a [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt) seed phrase.
</h3>

## principle

`f32` samples are collected from the audio input for 10 seconds and turned into an array of raw bytes, which is used as a `sha256` preimage.

the 32 bytes from the `sha256` hash are used as entropy for generating a BIP39 seed phrase of 24 words.

## ⚠️ warning⚠️

the author holds no liability in case you lose your coins.

use this tool at your own risk, ideally if you can read the source code and understand what it is doing.

## usage

```shell
$ cargo r
   Compiling seed_entropy_blower v0.1.0 (/Users/plebhash/develop/seed_entropy_blower)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/seed_entropy_blower`

🌬️🎙️ seed_entropy_blower 🎲🗝️

be prepared to generate some entropy by blowing into your mic in 5s.
1...
2...
3...
4...
5...
NOW! Keep blowing for 10s 🌬️🎙️
1...
2...
3...
4...
5...
6...
7...
8...
9...
10...

BIP39 seed phrase ✍️🗝️
position|word|number
1|student|1724
2|patrol|1290
3|happy|840
4|diary|491
5|purpose|1396
6|remain|1453
7|armor|96
8|method|1122
9|cheap|312
10|depart|471
11|frame|741
12|index|919
13|repeat|1462
14|alert|50
15|rice|1482
16|pear|1297
17|surround|1749
18|crash|404
19|gossip|808
20|ordinary|1251
21|empower|585
22|swing|1761
23|seek|1561
24|metal|1121
```