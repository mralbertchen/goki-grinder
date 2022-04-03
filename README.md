# goki-grinder
Inspired by `solana-key-grind`, this is a grinder written in rust for vanity addresses on Solana multisig implementations such as https://github.com/GokiProtocol/goki

## Using this tool

Clone the repo, then run `cargo run <your-pattern>` to grind for the pattern you want. The match is case-insensitive. Once found, the keypair json file will be written to your current directory.
