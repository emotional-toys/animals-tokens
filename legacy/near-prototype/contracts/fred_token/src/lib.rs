use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::test_utils::{get_logs, VMContextBuilder};
use near_sdk::{testing_env, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: UnorderedMap<u64, Token>,
    owner: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    id: u64,
    owner: AccountId,
    metadata: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            tokens: UnorderedMap::new(b"t".to_vec()),
            owner,
        }
    }

    pub fn mint(&mut self, token_id: u64, metadata: String) {
        let caller = env::predecessor_account_id();
        assert_eq!(caller, self.owner, "Only the contract owner can mint tokens");

        let token = Token {
            id: token_id,
            owner: caller.clone(),
            metadata,
        };

        self.tokens.insert(&token_id, &token);
    }

    pub fn transfer(&mut self, token_id: u64, new_owner: AccountId) {
        let caller = env::predecessor_account_id();
        let mut token = self.tokens.get(&token_id).expect("Token not found");

        assert_eq!(caller, token.owner, "Only the token owner can transfer it");

        token.owner = new_owner.clone();
        self.tokens.insert(&token_id, &token);
    }

    pub fn get_token(&self, token_id: u64) -> Option<Token> {
        self.tokens.get(&token_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alice() -> AccountId {
        "alice.near".to_string()
    }

    fn bob() -> AccountId {
        "bob.near".to_string()
    }

    #[test]
    fn test_mint() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(alice()).build());

        let mut contract = Contract::new(alice());
        contract.mint(1, "Token 1".to_string());

        let token = contract.get_token(1).unwrap();
        assert_eq!(token.id, 1);
        assert_eq!(token.owner, alice());
        assert_eq!(token.metadata, "Token 1");
    }

    #[test]
    fn test_transfer() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(alice()).build());

        let mut contract = Contract::new(alice());
        contract.mint(1, "Token 1".to_string());

        testing_env!(context.predecessor_account_id(alice()).build());
        contract.transfer(1, bob());

        let token = contract.get_token(1).unwrap();
        assert_eq!(token.owner, bob());
    }
}