use borsh:: { BorshSeralize, BorshDeserialize }; 
use solana_program:: {     
    program_pack::{ IsInitialized, Sealed },
    pubkey::Pubkey,
    clock::UnixTimestamp
}

#[derive(BorshSerialize, BorshDeserialize, Debug)] pub struct UserStakeInfo { pub is_initialized: bool, pub token_account: Pubkey, pub stake_start_time: UnixTimestamp, pub last_stake_redeem: UnixTimestamp, pub user_pubkey: Pubkey, pub stake_state: StakeState, }

impl UserStakeInfo { 
     pub const SIZE: usize = 1 + 32 + 64 + 64 + 32 + 1; 
    }

impl Sealed for UserStakeInfo { } impl IsInitialized for UserStakeInfo { fn is_initialized(&self) -> bool { self.is_initialized     }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)] pub enum StakeState {     Staked,
    Unstaked
}
