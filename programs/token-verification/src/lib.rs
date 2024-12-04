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
use thiserror::Error;

// Program ID and Fee Wallet
solana_program::declare_id!("TokenVerify111111111111111111111111111111");
pub const FEE_WALLET: &str = "6zkf4DviZZkpWVEh53MrcQV6vGXGpESnNXgAvU6KpBUH";

// Verification fees in lamports
pub const VERIFICATION_FEE: u64 = 500_000_000;  // 0.5 SOL
pub const DEX_LISTING_FEE: u64 = 1_000_000_000; // 1 SOL

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenVerificationInfo {
    pub mint_address: Pubkey,
    pub owner: Pubkey,
    pub is_verified: bool,
    pub dex_listed: bool,
    pub verification_time: i64,
    pub social_links: TokenSocialLinks,
    pub metrics: TokenMetrics,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenSocialLinks {
    pub website: String,
    pub twitter: String,
    pub telegram: String,
    pub discord: String,
    pub github: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetrics {
    pub total_holders: u64,
    pub market_cap: u64,
    pub total_supply: u64,
    pub circulating_supply: u64,
}

#[derive(FromPrimitive, Debug)]
pub enum VerificationInstruction {
    InitVerification,
    UpdateSocialLinks,
    UpdateMetrics,
    RequestDexListing,
    VerifyToken,
    RevokeVerification,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum VerificationError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Already verified")]
    AlreadyVerified,
    #[error("Insufficient fee")]
    InsufficientFee,
    #[error("Invalid authority")]
    InvalidAuthority,
    #[error("Not verified")]
    NotVerified,
}

impl From<VerificationError> for ProgramError {
    fn from(e: VerificationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VerificationInstruction::try_from_primitive(instruction_data[0])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        VerificationInstruction::InitVerification => {
            process_init_verification(program_id, accounts, &instruction_data[1..])
        }
        VerificationInstruction::UpdateSocialLinks => {
            process_update_social_links(program_id, accounts, &instruction_data[1..])
        }
        VerificationInstruction::UpdateMetrics => {
            process_update_metrics(program_id, accounts, &instruction_data[1..])
        }
        VerificationInstruction::RequestDexListing => {
            process_request_dex_listing(program_id, accounts)
        }
        VerificationInstruction::VerifyToken => {
            process_verify_token(program_id, accounts)
        }
        VerificationInstruction::RevokeVerification => {
            process_revoke_verification(program_id, accounts)
        }
    }
}

fn process_init_verification(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let verification_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let fee_wallet = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent = Rent::get()?;

    // Verify fee wallet
    if fee_wallet.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    // Transfer verification fee
    solana_program::program::invoke(
        &system_instruction::transfer(
            payer_account.key,
            fee_wallet.key,
            VERIFICATION_FEE,
        ),
        &[
            payer_account.clone(),
            fee_wallet.clone(),
            system_program.clone(),
        ],
    )?;

    let social_links = TokenSocialLinks {
        website: String::new(),
        twitter: String::new(),
        telegram: String::new(),
        discord: String::new(),
        github: String::new(),
    };

    let metrics = TokenMetrics {
        total_holders: 0,
        market_cap: 0,
        total_supply: 0,
        circulating_supply: 0,
    };

    let verification_info = TokenVerificationInfo {
        mint_address: *mint_account.key,
        owner: *payer_account.key,
        is_verified: false,
        dex_listed: false,
        verification_time: 0,
        social_links,
        metrics,
    };

    verification_info.serialize(&mut *verification_account.data.borrow_mut())?;

    Ok(())
}

fn process_update_social_links(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let verification_account = next_account_info(account_info_iter)?;

    let mut verification_info = TokenVerificationInfo::try_from_slice(&verification_account.data.borrow())?;
    
    if verification_info.owner != *owner_account.key {
        return Err(VerificationError::InvalidAuthority.into());
    }

    let social_links = TokenSocialLinks::try_from_slice(instruction_data)?;
    verification_info.social_links = social_links;
    verification_info.serialize(&mut *verification_account.data.borrow_mut())?;

    Ok(())
}

fn process_update_metrics(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_account = next_account_info(account_info_iter)?;
    let verification_account = next_account_info(account_info_iter)?;

    // Only program authority can update metrics
    if authority_account.key != program_id {
        return Err(VerificationError::InvalidAuthority.into());
    }

    let mut verification_info = TokenVerificationInfo::try_from_slice(&verification_account.data.borrow())?;
    let metrics = TokenMetrics::try_from_slice(instruction_data)?;
    verification_info.metrics = metrics;
    verification_info.serialize(&mut *verification_account.data.borrow_mut())?;

    Ok(())
}

fn process_request_dex_listing(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let verification_account = next_account_info(account_info_iter)?;
    let fee_wallet = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let mut verification_info = TokenVerificationInfo::try_from_slice(&verification_account.data.borrow())?;
    
    if !verification_info.is_verified {
        return Err(VerificationError::NotVerified.into());
    }

    if verification_info.owner != *owner_account.key {
        return Err(VerificationError::InvalidAuthority.into());
    }

    // Verify fee wallet
    if fee_wallet.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    // Transfer DEX listing fee
    solana_program::program::invoke(
        &system_instruction::transfer(
            owner_account.key,
            fee_wallet.key,
            DEX_LISTING_FEE,
        ),
        &[
            owner_account.clone(),
            fee_wallet.clone(),
            system_program.clone(),
        ],
    )?;

    verification_info.dex_listed = true;
    verification_info.serialize(&mut *verification_account.data.borrow_mut())?;

    Ok(())
}

fn process_verify_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_account = next_account_info(account_info_iter)?;
    let verification_account = next_account_info(account_info_iter)?;

    // Only program authority can verify tokens
    if authority_account.key != program_id {
        return Err(VerificationError::InvalidAuthority.into());
    }

    let mut verification_info = TokenVerificationInfo::try_from_slice(&verification_account.data.borrow())?;
    
    if verification_info.is_verified {
        return Err(VerificationError::AlreadyVerified.into());
    }

    verification_info.is_verified = true;
    verification_info.verification_time = solana_program::clock::Clock::get()?.unix_timestamp;
    verification_info.serialize(&mut *verification_account.data.borrow_mut())?;

    Ok(())
}

fn process_revoke_verification(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_account = next_account_info(account_info_iter)?;
    let verification_account = next_account_info(account_info_iter)?;

    // Only program authority can revoke verification
    if authority_account.key != program_id {
        return Err(VerificationError::InvalidAuthority.into());
    }

    let mut verification_info = TokenVerificationInfo::try_from_slice(&verification_account.data.borrow())?;
    verification_info.is_verified = false;
    verification_info.dex_listed = false;
    verification_info.serialize(&mut *verification_account.data.borrow_mut())?;

    Ok(())
}
