use std::sync::Arc;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::transport::TransportError;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signer},
};

use crate::*;

#[derive(Clone)]
pub struct AddinCookie {
    pub solana: Arc<solana::SolanaCookie>,
    pub program_id: Pubkey,
}

pub struct RegistrarCookie {
    pub address: Pubkey,
    pub authority: Pubkey,
    pub mint: MintCookie,
}

#[derive(Clone)]
pub struct VotingMintConfigCookie {
    pub mint: MintCookie,
}

pub struct VoterCookie {
    pub authority: Pubkey,
    pub voter_weight_record: Pubkey,
    pub voter_weight_record_bump: u8,
    pub token_owner_record: Pubkey,
}

impl AddinCookie {
    pub async fn create_registrar(
        &self,
        realm: &GovernanceRealmCookie,
        authority: &Keypair,
        payer: &Keypair,
    ) -> RegistrarCookie {
        let community_token_mint = realm.community_token_mint.pubkey.unwrap();

        let (registrar, registrar_bump) = Pubkey::find_program_address(
            &[
                &realm.realm.to_bytes(),
                b"registrar".as_ref(),
                &community_token_mint.to_bytes(),
            ],
            &self.program_id,
        );

        let data = anchor_lang::InstructionData::data(
            &plugin_boilerplate::instruction::CreateRegistrar { registrar_bump },
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &plugin_boilerplate::accounts::CreateRegistrar {
                registrar,
                governance_program_id: realm.governance.program_id,
                realm: realm.realm,
                realm_governing_token_mint: community_token_mint,
                realm_authority: realm.authority,
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
                rent: solana_program::sysvar::rent::id(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        // clone the user secret
        let signer1 = Keypair::from_base58_string(&payer.to_base58_string());
        let signer2 = Keypair::from_base58_string(&authority.to_base58_string());

        self.solana
            .process_transaction(&instructions, Some(&[&signer1, &signer2]))
            .await
            .unwrap();

        RegistrarCookie {
            address: registrar,
            authority: realm.authority,
            mint: realm.community_token_mint.clone(),
        }
    }

    pub async fn create_voter_weight_record(
        &self,
        registrar: &RegistrarCookie,
        token_owner_record: &TokenOwnerRecordCookie,
        authority: &Keypair,
        payer: &Keypair,
    ) -> VoterCookie {
        let (voter_weight_record, voter_weight_record_bump) = Pubkey::find_program_address(
            &[
                &registrar.address.to_bytes(),
                b"voter-weight-record".as_ref(),
                &authority.pubkey().to_bytes(),
            ],
            &self.program_id,
        );

        let data =
            anchor_lang::InstructionData::data(&plugin_boilerplate::instruction::CreateVoterWeightRecord {
                voter_weight_record_bump,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &plugin_boilerplate::accounts::CreateVoterWeightRecord {
                voter_weight_record,
                registrar: registrar.address,
                voter_authority: authority.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
                rent: solana_program::sysvar::rent::id(),
                instructions: solana_program::sysvar::instructions::id(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        // clone the secrets
        let signer1 = Keypair::from_base58_string(&payer.to_base58_string());
        let signer2 = Keypair::from_base58_string(&authority.to_base58_string());

        self.solana
            .process_transaction(&instructions, Some(&[&signer1, &signer2]))
            .await
            .unwrap();

        VoterCookie {
            authority: authority.pubkey(),
            voter_weight_record,
            voter_weight_record_bump,
            token_owner_record: token_owner_record.address,
        }
    }

    pub fn update_voter_weight_record_instruction(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
    ) -> Instruction {
        let data = anchor_lang::InstructionData::data(
            &plugin_boilerplate::instruction::UpdateVoterWeightRecord {
                voter_weight_record_bump: voter.voter_weight_record_bump,
            },
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &plugin_boilerplate::accounts::UpdateVoterWeightRecord {
                registrar: registrar.address,
                voter: voter.authority,
                voter_weight_record: voter.voter_weight_record,
                system_program: solana_sdk::system_program::id(),
            },
            None,
        );

        Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }
    }

    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
    ) -> std::result::Result<plugin_boilerplate::state::VoterWeightRecord, TransportError> {
        let instructions = vec![self.update_voter_weight_record_instruction(registrar, voter)];

        self.solana.process_transaction(&instructions, None).await?;

        Ok(self
            .solana
            .get_account::<plugin_boilerplate::state::VoterWeightRecord>(
                voter.voter_weight_record,
            )
            .await)
    }

    #[allow(dead_code)]
    pub async fn set_time_offset(
        &self,
        registrar: &RegistrarCookie,
        authority: &Keypair,
        time_offset: i64,
    ) {
        let data =
            anchor_lang::InstructionData::data(&plugin_boilerplate::instruction::SetTimeOffset {
                time_offset,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &plugin_boilerplate::accounts::SetTimeOffset {
                registrar: registrar.address,
                realm_authority: authority.pubkey(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        // clone the secrets
        let signer = Keypair::from_base58_string(&authority.to_base58_string());

        self.solana
            .process_transaction(&instructions, Some(&[&signer]))
            .await
            .unwrap();
    }
}