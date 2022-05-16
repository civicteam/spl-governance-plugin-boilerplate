use anchor_lang::prelude::*;

#[error_code]
pub enum BoilerplateError {
    // 6000 / 0x1770
    #[msg("")]
    InvalidAuthority,
    // 6001 / 0x1771
    #[msg("")]
    InvalidRealmAuthority,
    // 6002 / 0x1772
    #[msg("")]
    VoterWeightOverflow,
    // 6003 / 0x1773
    #[msg("")]
    InternalProgramError,
    // 6004 / 0x1774
    #[msg("")]
    DebugInstruction,
    // 6005 / 0x1775
    #[msg("")]
    ForbiddenCpi,
}
