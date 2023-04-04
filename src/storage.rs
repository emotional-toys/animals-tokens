impl StorageManagement for Contract {
    #[allow(unused_variables)]
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }
}