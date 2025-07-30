// programs/asset_nft/src/lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("ASSETpL1aToL1aL1aL1aL1aL1aL1aL1aL1aL1aL1aL1"); // Placeholder Program ID

#[program]
pub mod asset_nft {
    use super::*;

    // Create a new Asset NFT representing a real-world asset
    pub fn create_asset(
        ctx: Context<CreateAsset>,
        name: String,
        description: String,
        estimated_value: u64,
        metadata_uri: String, // Link to IPFS with photos, documents
    ) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.owner = *ctx.accounts.owner.key;
        asset.name = name;
        asset.description = description;
        asset.estimated_value = estimated_value;
        asset.metadata_uri = metadata_uri;
        asset.verification_level = VerificationLevel::SelfDeclared as u8;
        asset.bump = *ctx.bumps.get("asset").unwrap();
        
        msg!("New Asset NFT created: {}", asset.name);
        Ok(())
    }

    // Allow another user to vouch for the authenticity of an asset
    pub fn verify_by_community(ctx: Context<VerifyAsset>) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        // Logic to prevent owner from verifying their own asset
        require_keys_neq!(asset.owner, *ctx.accounts.verifier.key, AssetError::OwnerCannotVerify);

        asset.verification_level = VerificationLevel::CommunityVerified as u8;
        
        msg!("Asset {} verified by community member {}", asset.name, ctx.accounts.verifier.key());
        Ok(())
    }
}

#[account]
pub struct Asset {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub estimated_value: u64, // In USD cents for precision
    pub metadata_uri: String,
    pub verification_level: u8, // 0: SelfDeclared, 1: CommunityVerified, 2: ProfessionalVerified
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 100 + 4 + 200 + 8 + 4 + 200 + 1 + 1, // Estimate space
        seeds = [b"asset", owner.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub asset: Account<'info, Asset>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, Mint>, // The NFT mint account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyAsset<'info> {
    #[account(mut, has_one = owner)]
    pub asset: Account<'info, Asset>,
    pub owner: SystemAccount<'info>, // The owner of the asset being verified
    pub verifier: Signer<'info>, // The community member performing the verification
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum VerificationLevel {
    SelfDeclared,
    CommunityVerified,
    ProfessionalVerified,
}

#[error_code]
pub enum AssetError {
    #[msg("The owner of an asset cannot verify it.")]
    OwnerCannotVerify,
}