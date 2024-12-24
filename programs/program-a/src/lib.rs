use anchor_lang::prelude::*;
use program_b::program::ProgramB;
declare_id!("HhD8aXNnfQ3mAZZZYoz6WtCtW5ZC25HuM5xNk7a8szRv");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: program A");

        let pda_address: Pubkey = _ctx.accounts.pda_account.key();
        let signer_address: Pubkey = _ctx.accounts.signer.key();
        let bump = _ctx.bumps.pda_account;

        let instruction =
            &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);

        let account_infos = [
            _ctx.accounts.pda_account.to_account_info(), //sender
            _ctx.accounts.signer.to_account_info(),      //receiver
            _ctx.accounts.system_program.to_account_info(),
        ];

        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, &account_infos, signer_seeds)?;

        // Now Call Program B 
        //new_with_signer is the same as invoke_signed
        let cpi_context = CpiContext::new_with_signer(
            _ctx.accounts.program_b.to_account_info(), 
            program_b::cpi::accounts::Initialize{pda_account: _ctx.accounts.pda_account.to_account_info()}, 
            signer_seeds
        );

        program_b::cpi::initialize(cpi_context)?; // ? -> propogates error

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: pda
    #[account(
        mut,
        seeds = [b"pda", signer.key().as_ref()],
        bump, // use canoncical bump, starts from 255 and loops to find correct bump
    )]
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>, //receiver of transfer
    pub system_program: Program<'info, System>, //CPI to system program
    pub program_b: Program<'info, ProgramB>,    //CPI to program_b
}
