// programs/blago_dao/src/lib.rs -- ИСПРАВЛЕННАЯ ВЕРСИЯ
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("5XQDfM2JuVoeeJpFA1QTkLeiuCTLTYrpd4hQ54tbVqJy");

#[program]
pub mod blago_dao {
    use super::*;
    pub fn create_proposal( ctx: Context<CreateProposal>, title: String, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.is_active = true;
        Ok(())
    }
    pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, vote_type: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.is_active, DaoError::ProposalInactive);
        let vote_weight = ctx.accounts.user_good_account.amount;
        require!(vote_weight > 0, DaoError::NoVotingPower);
        if vote_type {
            proposal.votes_for += vote_weight;
        } else {
            proposal.votes_against += vote_weight;
        }
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
}
#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + (4 + title.len()) + (4 + description.len()) + 8 + 8 + 1,
        seeds = [b"proposal", creator.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
    pub user_good_account: Account<'info, TokenAccount>,
}
#[error_code]
pub enum DaoError { #[msg("This proposal is not active for voting.")] ProposalInactive, #[msg("You have no GOOD tokens to vote with.")] NoVotingPower, }