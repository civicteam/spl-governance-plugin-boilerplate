import { Program, Provider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { PluginBoilerplate, IDL } from '../target/types/plugin_boilerplate';

export const PLUGIN_ID = new PublicKey(
  '6DNF4tFLynLJhJh78dCNkft9A5Sd49cmypD2SZ4Hfu7S',
);

export class PluginClient {
  constructor(
    public program: Program<PluginBoilerplate>,
    public devnet?: boolean,
  ) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<PluginClient> {
    // alternatively we could fetch from chain
    // const idl = await Program.fetchIdl(VSR_ID, provider);
    const idl = IDL;

    return new PluginClient(
      new Program<PluginBoilerplate>(
        idl as PluginBoilerplate,
        PLUGIN_ID,
        provider,
      ),
      devnet,
    );
  }
}
