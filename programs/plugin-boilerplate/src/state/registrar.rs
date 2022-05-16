use anchor_lang::prelude::*;

// Generate a VoteWeightRecord Anchor wrapper, owned by the current program.
// VoteWeightRecords are unique in that they are defined by the SPL governance
vote_weight_record!(crate::ID);

/// Instance of a vote-weight registrar.
#[account(zero_copy)]
#[derive(Default)]
pub struct Registrar {
    pub governance_program_id: Pubkey,
    pub realm: Pubkey,
    pub realm_governing_token_mint: Pubkey,
    pub realm_authority: Pubkey,

    // Add your fields here
    
    /// Debug only: time offset, to allow tests to move forward in time.
    pub time_offset: i64,
    pub bump: u8,
    // pub reserved2: [u8; 7],
    // pub reserved3: [u64; 11], // split because `Default` does not support [u8; 95]
}
// const_assert!(std::mem::size_of::<Registrar>() == 5 * 32 + 4 * 152 + 8 + 1 + 95);
const_assert!(std::mem::size_of::<Registrar>() == 4 * 32 + 8 + 1 + 7); // Anchor adds 7 bytes of what appears to be padding, to make it % 8 == 0
const_assert!(std::mem::size_of::<Registrar>() % 8 == 0);

impl Registrar {
    pub fn clock_unix_timestamp(&self) -> i64 {
        Clock::get()
            .unwrap()
            .unix_timestamp
            .checked_add(self.time_offset)
            .unwrap()
    }

    pub fn max_vote_weight(&self) -> Result<u64> {
        // Custom max voter weight calculation
        Ok(1000)
    }
}

#[macro_export]
macro_rules! registrar_seeds {
    ( $registrar:expr ) => {
        &[
            $registrar.realm.as_ref(),
            b"registrar".as_ref(),
            $registrar.realm_governing_token_mint.as_ref(),
            &[$registrar.bump],
        ]
    };
}

pub use registrar_seeds;
use crate::vote_weight_record;
