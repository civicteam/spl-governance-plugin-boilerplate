use crate::state::*;
use anchor_lang::prelude::*;

// Remaining accounts should all the token mints that have registered
// exchange rates.
#[derive(Accounts)]
pub struct UpdateMaxVoteWeight<'info> {
    pub registrar: AccountLoader<'info, Registrar>,
    /// CHECK: TODO: SPL governance has not yet implemented this.
    pub max_vote_weight_record: UncheckedAccount<'info>,
}

/// Calculates the max vote weight for the registry.
pub fn update_max_vote_weight(ctx: Context<UpdateMaxVoteWeight>) -> Result<()> {
    let registrar = &ctx.accounts.registrar.load()?;

    let _max_vote_weight = registrar.max_vote_weight()?;
    // TODO: SPL governance has not yet implemented this feature.
    //       When it has, probably need to write the result into an account,
    //       similar to VoterWeightRecord.
    Ok(())
}
