use solana_program:: {     account_info:: { AccountInfo, next_account_info },
    entrypoint::ProgramResult,
    pubkey::Pubkey,
}
use crate::instruction::StakeInstruction;
pub fn process_instruction(     program_id: &Pubkey,
    accounts: &[AccountInfo],
instruction_data: &[u8] ) -> ProgramResult {
let instruction = StakeInstruction::unpack(instruction_data)?;
match instruction {         StakeInstruction::InitializeStakeAccount => process_initialize_stake_account(program_id, accounts),
        StakeInstruction::Stake => process_stake(program_id, accounts),
        StakeInstruction::Redeem => process_redeem(program_id, accounts),
        StakeInstruction::Unstake => process_unstake(program_id, accounts)
    }
}

accounts: &[AccountInfo]
) -> ProgramResult {
let account_info_iter = &mut accounts.iter(); let user = next_account_info(account_info_iter)?; let nft_token = next_account_info(account_info_iter)?; let stake_state = next_account_info(account_info_iter)?; let system_program = next_account_info(account_info_iter)?;
Ok(()) }

fn process_stake(     program_id: &Pubkey,
    accounts: &[AccountInfo]
) -> ProgramResult {
Ok(()) }

fn process_redeem(     program_id: &Pubkey,
    accounts: &[AccountInfo]
) -> ProgramResult {
Ok(()) }

fn process_unstake(     program_id: &Pubkey,
    accounts: &[AccountInfo]
) -> ProgramResult {
Ok(()) }
