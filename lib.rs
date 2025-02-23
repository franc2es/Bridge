use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    sysvar::{rent::Rent, Sysvar},
    program_error::ProgramError,
};

use metaplex_token_metadata::state::{Metadata, MetadataAccount};

// 一个简化的示例，销毁NFT并创建跨链请求
pub fn process_nft_burn(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    nft_mint_pubkey: Pubkey, // NFT mint的public key
    bridge_program_pubkey: Pubkey, // 跨链桥的合约public key
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let metadata_account = next_account_info(account_info_iter)?; // Metaplex的Metadata账户
    let mint_account = next_account_info(account_info_iter)?; // Solana NFT的Mint账户

    // 检查账户是否是可销毁的NFT
    if *mint_account.key != nft_mint_pubkey {
        msg!("Invalid NFT mint account");
        return Err(ProgramError::InvalidAccountData);
    }

    // 获取Metadata数据
    let metadata_data = Metadata::from_account_info(metadata_account)?;
    if !metadata_data.is_initialized() {
        msg!("NFT metadata is not initialized");
        return Err(ProgramError::UninitializedAccount);
    }

    // 销毁NFT操作
    msg!("Burning NFT with mint: {}", nft_mint_pubkey);
    // 需要调用Metaplex的销毁逻辑（假设Metaplex SDK提供销毁功能）

    // 跨链请求（向目标链发送信息）
    msg!("Sending cross-chain request to: {}", bridge_program_pubkey);

    // 需要实现通过Solana系统调用跨链桥（如Wormhole等）发送跨链请求的逻辑。

    Ok(())
}
