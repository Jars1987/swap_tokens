# Token Swap - A Solana Program

This program is designed for the user to make an offer where it's specified the
amount and token he is going to offer and the amount and token he wants to
receive. The tokens are then sent to an escrow (Vault) and the offer details are
saved in the Offer Account. Finally when the second user takes the offer, the
program will send the second user tokens (the amount and token desired but the
used that made the offer) and send them directly to the maker of the offer. To
finish the user that took the offer will withdraw the tokens from the vault and
close the vault account.

## Program Structure

The instruction handlers make_offers and take_offer will live in the
instructions folder. They will be called later inside our program that is in
lib.rs file. The intrsuctions folder will also have a shared folder when we
store a helper function (transfer_tokens) to avoid repetition. Our Offer
account, where our offer (data) will be saved lives in our offer.rs file inside
the state folder.

- Instructions //_instruction handlers_
  - Shared // _helper function - transfer_tokens_
  - Make Offer
    - Accounts // _Maker(signer), token_mint (a & b), maker_token_acccount_a,
      offer, vault, system program, token program, associated account_
    - Functions //_ end_offered_tokens_to_vault & save_offer_
  - Take Ofeer
    - Accounts /// _Taker(signer), Maker, token_mint (a & b),
      taker_token_Account (a & b), maker_token_account_b, offer, vault, system
      program, token program, associated account_
    - Functions // _send_wanted_tokens_to_maker & withdraw_and_close_vault_
- State
  - offer.rs
- constants
- lib.rs
