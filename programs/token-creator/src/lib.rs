use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction as token_instruction;
use thiserror::Error;

// Program ID and Fee Wallet
solana_program::declare_id!("TokenCreator111111111111111111111111111111");
pub const FEE_WALLET: &str = "6zkf4DviZZkpWVEh53MrcQV6vGXGpESnNXgAvU6KpBUH";

// Fee structure (in lamports)
pub const BASE_TOKEN_CREATION_FEE: u64 = 100_000_000; // 0.1 SOL base fee
pub const MINT_AUTHORITY_FEE: u64 = 50_000_000;      // 0.05 SOL additional if mint authority is enabled
pub const FREEZE_AUTHORITY_FEE: u64 = 50_000_000;    // 0.05 SOL additional if freeze authority is enabled

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenCreationParams {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub enable_mint: bool,
    pub enable_freeze: bool,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum TokenCreatorError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Insufficient fee")]
    InsufficientFee,
}

impl From<TokenCreatorError> for ProgramError {
    fn from(e: TokenCreatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let params = TokenCreationParams::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let account_info_iter = &mut accounts.iter();
    
    let payer_account = next_account_info(account_info_iter)?;
    let fee_wallet = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    // Verify fee wallet
    if fee_wallet.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    // Calculate total fee
    let mut total_fee = BASE_TOKEN_CREATION_FEE;
    if params.enable_mint {
        total_fee = total_fee.checked_add(MINT_AUTHORITY_FEE)
            .ok_or(ProgramError::Overflow)?;
    }
    if params.enable_freeze {
        total_fee = total_fee.checked_add(FREEZE_AUTHORITY_FEE)
            .ok_or(ProgramError::Overflow)?;
    }

    // Transfer fee to the fee wallet
    solana_program::program::invoke(
        &system_instruction::transfer(
            payer_account.key,
            fee_wallet.key,
            total_fee,
        ),
        &[
            payer_account.clone(),
            fee_wallet.clone(),
            system_program.clone(),
        ],
    )?;

    msg!("Fee of {} lamports transferred to fee wallet", total_fee);

    // Create mint account
    let rent = &Rent::from_account_info(rent_sysvar)?;
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    solana_program::program::invoke(
        &system_instruction::create_account(
            payer_account.key,
            mint_account.key,
            mint_rent,
            spl_token::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[
            payer_account.clone(),
            mint_account.clone(),
            system_program.clone(),
        ],
    )?;

    // Initialize mint
    solana_program::program::invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            payer_account.key,
            if params.enable_freeze { Some(payer_account.key) } else { None },
            params.decimals,
        )?,
        &[
            mint_account.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    // If initial supply > 0, mint tokens to the payer
    if params.total_supply > 0 {
        // Create token account for payer
        let payer_token_account = next_account_info(account_info_iter)?;
        
        solana_program::program::invoke(
            &token_instruction::initialize_account(
                token_program.key,
                payer_token_account.key,
                mint_account.key,
                payer_account.key,
            )?,
            &[
                payer_token_account.clone(),
                mint_account.clone(),
                payer_account.clone(),
                rent_sysvar.clone(),
            ],
        )?;

        // Mint initial supply
        solana_program::program::invoke(
            &token_instruction::mint_to(
                token_program.key,
                mint_account.key,
                payer_token_account.key,
                payer_account.key,
                &[],
                params.total_supply,
            )?,
            &[
                mint_account.clone(),
                payer_token_account.clone(),
                payer_account.clone(),
            ],
        )?;
    }

    msg!("Token created successfully with mint address: {}", mint_account.key);
    Ok(())
}
