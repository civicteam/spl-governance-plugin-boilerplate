# Description

plugin-boilerplate is a template plugin repo for Solana's
[spl-governance program](https://github.com/solana-labs/solana-program-library/tree/master/governance).

Use this repo to develop plugins for the spl-governance program. Typically, the places you will need to change are:
- update-voter-weight instruction
- update-max-voter-weight instruction
- The registry struct
- Optionally, replace the create-voter-weight instruction with a create-voter instruction and update the voter struct 

# Development

## Rust
* Built and developed using - rust stable(`rustc 1.57.0 (f1edd0429 2021-11-29)`)
* Run rust based tests - `cargo test-bpf`
* `run-generate-anchor-types.sh` generates latest anchor types file and writes to `./plugin_boilerplate.ts`
* To install the typescript client, do - `yarn add @civic/plugin-boilerplate-client`
* usage

## Node/Typescript
* Built and developed using - node (`v16.13.1`)
* Usage
```
import { Provider, Wallet } from '@project-serum/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import { VsrClient } from '@civic/plugin-boilerplate-client';

async function main() {
  const options = Provider.defaultOptions();
  const connection = new Connection('https://api.devnet.solana.com', options);
  const wallet = new Wallet(Keypair.generate());
  const provider = new Provider(connection, wallet, options);
  const client = await PluginClient.connect(provider, true);
```

<img width="708" alt="image" src="https://user-images.githubusercontent.com/89031858/148725266-29459e80-623e-45c4-952d-5d9d1f0f15bc.png">

# Instruction Overview

## Setup

- [`CreateRegistrar`](programs/plugin-boilerplate/src/instructions/create_registrar.rs)

  Creates a Registrar account for a governance realm.

## Usage

- [`CreateVoterWeightRecord`](programs/plugin-boilerplate/src/instructions/create_voter_weight_record)

  Create a new voter weight record with no weight.

- [`UpdateVoterWeightRecord`](programs/plugin-boilerplate/src/instructions/update_voter_weight_record.rs)

  Write the current voter weight to the account that spl-governance can read to
  prepare for voting. The voter weight in this boilerplate is fixed to 1000

## Special

- [`UpdateMaxVoteWeight`](programs/plugin-boilerplate/src/instructions/update_max_vote_weight.rs)

  Unfinished instruction for telling spl-governance about the total maximum vote weight.

- [`SetTimeOffset`](programs/plugin-boilerplate/src/instructions/set_time_offset.rs)

  Debug instruction for advancing time in tests. Not usable.


# License

This code is currently not free to use while in development.


# References:
* [spl-governance](https://github.com/solana-labs/solana-program-library/tree/master/governance)
