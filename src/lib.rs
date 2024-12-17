use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey, // Program ID
    accounts: &[AccountInfo], // List of accounts
    instruction_data: &[u8], // Instruction-specific data
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // 获取发送者账户（只有一个账户）
    let user_account = next_account_info(account_info_iter)?;
    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 解码存款金额
    let deposit_amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // 存款逻辑：将指定金额从用户账户转账到程序账户
    msg!("处理存款...");
    **user_account.try_borrow_mut_lamports()? -= deposit_amount; // 从用户账户扣除金额
    **user_account.try_borrow_mut_lamports()? += deposit_amount; // 存入到程序账户

    msg!("存款完成");
    Ok(())
}
