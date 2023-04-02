# turbo-rs

Crypto utils, mainly for ethereum for now.

```
Usage: turbo [OPTIONS] <COMMAND>

Commands:
  key   hdwallet key generator for bip32 with bip39
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Turn debugging information on
  -h, --help        Print help
  -V, --version     Print version
```

## <kbd>key</kbd>

This command provides a hdwallet key generator follows bip32 with bip39.

* This implementation uses the `secp256k1` algorithm.
* For the derivation path of bip39, it uses `m/44'/60'/0'/0` from metamask. 

```
$ turbo key
hdwallet key generator for bip32 with bip39

Usage: turbo key <COMMAND>

Commands:
  generate  Generate new wallet
  derive    Derive new keys from {seed,mnemonic}
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

examples

```
$ turbo key generate
MNEMONIC: kitchen bag snow forget aspect bargain swim predict wrestle prepare coast avocado
Account {
    key: "0xc1d3bef2ee0f33c6262cc7607a08068b5730019913ec56018320123beb92211a",
    address: "0x2DB6CAA0385EA1494CB4FA48C9CE1812138818A4",
}

$ turbo key derive "kitchen bag snow forget aspect bargain swim predict wrestle prepare coast avocado" -c 5
Account {
    key: "0xe473c4656f0ec2ee72e91ab16fd0c27945d1c11a592d7f0f470dc58ac9034145",
    address: "0xE2C08263BBDD148544DC1A617B231F1FB63A4FD5",
}
Account {
    key: "0xfb785bb7384a768bb81399ad639685c278b5c3290902247414b46c744e056026",
    address: "0x8770EE870A616100DC042CEE0B888F1D65427371",
}
Account {
    key: "0x6e34171d24949ae135e1b7c60507dcf1b48493e365ed5b2a0e45ae4d1a594603",
    address: "0x23568167964327AFB7268DC32177BC44C476A382",
}
Account {
    key: "0x687fe0510399e49a3aa2bea5bbd81d7a92cb371f0e3ec9f78ffcedea3389271b",
    address: "0x38ACF312E8D93869E0C309C8B673C2C9DCF391EC",
}
Account {
    key: "0xbd45a73bdd4f6bca0c9d95fc791ff8f368033d64b235d5d436ef42b6b5d9ff03",
    address: "0xD93911ADDCFF525DA70EA0708EC0C0E108EC6248",
}
```
