// programs/blago_dao/src/lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("DAOpL1aToL1aL1aL1aL1aL1aL1aL1aL1aL1aL1aL1aL1"); // Placeholder Program ID

#[program]
pub mod blago_dao {
    use super::*;

    // Create a new proposal for the community to vote on
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.is_active = true;
        proposal.bump = *ctx.bumps.get("proposal").unwrap();
        
        msg!("New proposal created: {}", proposal.title);
        Ok(())
    }

    // Vote on an active proposal. The vote weight is the user's GOOD token balance.
    pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, vote_type: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.is_active, DaoError::ProposalInactive);

        let vote_weight = ctx.accounts.user_good_account.amount;
        require!(vote_weight > 0, DaoError::NoVotingPower);

        if vote_type { // true = For, false = Against
            proposal.votes_for += vote_weight;
            msg!("Voted FOR proposal '{}' with weight {}", proposal.title, vote_weight);
        } else {
            proposal.votes_against += vote_weight;
            msg!("Voted AGAINST proposal '{}' with weight {}", proposal.title, vote_weight);
        }

        // To prevent double voting, we would typically create a separate "VoteRecord" account.
        // For simplicity, this logic is omitted here.

        Ok(())
    }
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 4 + 100 + 4 + 500 + 8 + 8 + 1 + 1, // Estimate
        seeds = [b"proposal", creator.key().as_ref()], // Simplified
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>, // Must be a member of the Council
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
    // This account proves the voter's reputation (GOOD balance)
    pub user_good_account: Account<'info, TokenAccount>,
}

#[error_code]
pub enum DaoError {
    #[msg("This proposal is not active for voting.")]
    ProposalInactive,
    #[msg("You have no GOOD tokens to vote with.")]
    NoVotingPower,
}