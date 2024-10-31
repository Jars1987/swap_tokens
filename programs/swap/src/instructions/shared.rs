use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};


//we do this shared fnction so we avoid repeating this code later on
pub fn transfer_tokens<'info> (
  //token account is fine to be specific to either the old token program or the new token extensions program but for both at the same time we use InterfaceAccount
  from: &InterfaceAccount<'info, TokenAccount>, 
  to: &InterfaceAccount<'info, TokenAccount>,
  amount: &u64,
  mint: &InterfaceAccount<'info, Mint>,
  authority: &Signer<'info>,
  token_program: &Interface<'info, TokenInterface>
) -> Result<()> {

  //this is our transfer options struct
  let transfer_accounts_options = TransferChecked {
    from: from.to_account_info(),
    mint: mint.to_account_info(),
    to: to.to_account_info(),
    authority: authority.to_account_info(),
  };

  //this is the program we are about to call
  let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);

  // calling the program
  transfer_checked(cpi_context, *amount, mint.decimals)?;

  Ok(())
}