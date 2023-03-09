use solana_program::{     
    account_info::{ AccountInfo, next_account_info },
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    sysvar::{ rent::Rent, Sysvar },
    clock::Clock,
    program_pack::IsInitialized,
    system_instruction,
    program::invoke_signed,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError
}
use borsh::BorshSerialize; use crate::instruction::StakeInstruction; use crate::error::StakeError; use create::state::{ UserStakeInfo, StakeState };
fn process_initialize_stake_account(     program_id: &Pubkey,
    accounts: &[AccountInfo]
) -> ProgramResult {
let account_info_iter = &mut accounts.iter(); let user = next_account_info(account_info_iter)?; let nft_token = next_account_info(account_info_iter)?; let stake_state = next_account_info(account_info_iter)?; let system_program = next_account_info(account_info_iter)?;
let (stake_state_pda, bump_seed) = Pubkey::find_program_address(         &[user.key.as_ref(), nft_token_account.key.as_ref()],
        program_id
    );

let rent = Rent::get()?; let rent_lamports = rent.minimum_balance(UserStakeInfo::SIZE);
msg!("Creating state account at {:?}", stake_state_pda);     invoke_signed(
        &system_instruction::create_account(
            user.key,
            stake_state.key,
            rent_lamports,
            UserStakeInfo::SIZE.try_into().unwrap(),
            program_id
        ),
        &[user.clone(), stake_state.clone(), system_program.clone()],
        &[&[
            user.key.as_ref(),
            nft_token_account.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

let mut account_data = try_from_slice_unchecked::<UserStakeInfo>(&stake_state.data.borrow()).unwrap()
if account_data.is_initialized() { msg!("Account already initialized");
    return Err(ProgramError::AccountAlreadyInitialized);    
 }

    account_data.token_account = *nft_token_account.key;
    account_data.user_pubkey = *user.key;
    account_data.stake_state = StakeState::Unstaked;
account_data.is_initialized = true;
account_data.serialize(&mut &mut stake_state.data.borrow_mut()[..])?;
Ok(()) }
