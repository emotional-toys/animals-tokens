pub struct FungibleTokenMetadata {
    /*
        FILL THIS IN
    */
}

pub trait FungibleTokenMetadataProvider {
    // View call for returning the contract metadata
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }
}