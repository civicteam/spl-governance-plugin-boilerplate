use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(voter_weight_record_bump: u8)]
pub struct UpdateVoterWeightRecord<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    /// In the boilerplate, the voter is just any public key
    /// CHECK: No checks needed here. TODO - I think?
    pub voter: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter.key().as_ref()],
        bump = voter_weight_record_bump,
        constraint = voter_weight_record.realm == registrar.load()?.realm,
        constraint = voter_weight_record.governing_token_mint == registrar.load()?.realm_governing_token_mint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    pub system_program: Program<'info, System>,
}

pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>, _voter_weight_record_bump: u8) -> Result<()> {
    let _registrar = &ctx.accounts.registrar.load()?;
    
    // custom logic here
    let weight = 1000;
    
    let record = &mut ctx.accounts.voter_weight_record;
    record.voter_weight = weight;
    record.voter_weight_expiry = Some(Clock::get()?.slot);

    Ok(())
}
