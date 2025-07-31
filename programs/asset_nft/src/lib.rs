// programs/asset_nft/src/lib.rs -- ИСПРАВЛЕННАЯ ВЕРСИЯ
use anchor_lang::prelude::*;

declare_id!("5oNmieefkw6tLGrkeYwsr8ST5YAhES9t4K7DNM8W81pw");

#[program]
pub mod asset_nft {
    use super::*;
    pub fn create_asset( ctx: Context<CreateAsset>, name: String, description: String, estimated_value: u64, metadata_uri: String) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.owner = *ctx.accounts.owner.key;
        asset.name = name;
        asset.description = description;
        asset.estimated_value = estimated_value;
        asset.metadata_uri = metadata_uri;
        asset.verification_level = VerificationLevel::SelfDeclared as u8;
        Ok(())
    }
    pub fn verify_by_community(ctx: Context<VerifyAsset>) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        require_keys_neq!(asset.owner, *ctx.accounts.verifier.key, AssetError::OwnerCannotVerify);
        asset.verification_level = VerificationLevel::CommunityVerified as u8;
        Ok(())
    }
}
#[account]
pub struct Asset {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub estimated_value: u64,
    pub metadata_uri: String,
    pub verification_level: u8,
}
#[derive(Accounts)]
#[instruction(name: String, description: String, metadata_uri: String)]
pub struct CreateAsset<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + (4 + name.len()) + (4 + description.len()) + 8 + (4 + metadata_uri.len()) + 1,
        seeds = [b"asset", owner.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub asset: Account<'info, Asset>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct VerifyAsset<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    pub verifier: Signer<'info>,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VerificationLevel { SelfDeclared, CommunityVerified, ProfessionalVerified }
#[error_code]
pub enum AssetError { #[msg("The owner of an asset cannot verify it.")] OwnerCannotVerify, }