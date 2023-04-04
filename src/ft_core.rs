impl FungibleTokenCore for Contract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        /*
            FILL THIS IN
        */
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    fn ft_total_supply(&self) -> U128 {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }
}

#[ext_contract(ext_ft_receiver)]
pub trait FungibleTokenReceiver {
    /// Called by fungible token contract after `ft_transfer_call` was initiated by
    /// `sender_id` of the given `amount` with the transfer message given in `msg` field.
    /// The `amount` of tokens were already transferred to this contract account and ready to be used.
    ///
    /// The method must return the amount of tokens that are *not* used/accepted by this contract from the transferred
    /// amount. Examples:
    /// - The transferred amount was `500`, the contract completely takes it and must return `0`.
    /// - The transferred amount was `500`, but this transfer call only needs `450` for the action passed in the `msg`
    ///   field, then the method must return `50`.
    /// - The transferred amount was `500`, but the action in `msg` field has expired and the transfer must be
    ///   cancelled. The method must return `500` or panic.
    ///
    /// Arguments:
    /// - `sender_id` - the account ID that initiated the transfer.
    /// - `amount` - the amount of tokens that were transferred to this account in a decimal string representation.
    /// - `msg` - a string message that was passed with this transfer call.
    ///
    /// Returns the amount of unused tokens that should be returned to sender, in a decimal string representation.
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl Contract {
    // Finalize an `ft_transfer_call` chain of cross-contract calls.
    //
    // The `ft_transfer_call` process:
    //
    // 1. Sender calls `ft_transfer_call` on FT contract
    // 2. FT contract transfers `amount` tokens from sender to receiver
    // 3. FT contract calls `ft_on_transfer` on receiver contract
    // 4+. [receiver contract may make other cross-contract calls]
    // N. FT contract resolves promise chain with `ft_resolve_transfer`, and may
    //    refund sender some or all of original `amount`
    //
    // Requirements:
    // * Contract MUST forbid calls to this function by any account except self
    // * If promise chain failed, contract MUST revert token transfer
    // * If promise chain resolves with a non-zero amount given as a string,
    //   contract MUST return this amount of tokens to `sender_id`
    //
    // Arguments:
    // * `sender_id`: the sender of `ft_transfer_call`
    // * `receiver_id`: the `receiver_id` argument given to `ft_transfer_call`
    // * `amount`: the `amount` argument given to `ft_transfer_call`
    //
    // Returns a string representing a string version of an unsigned 128-bit
    // integer of how many total tokens were spent by sender_id. Example: if sender
    // calls `ft_transfer_call({ "amount": "100" })`, but `receiver_id` only uses
    // 80, `ft_on_transfer` will resolve with `"20"`, and `ft_resolve_transfer`
    // will return `"80"`.
    #[private]
    pub fn ft_resolve_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }