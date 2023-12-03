use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize};
use near_sdk::{near_bindgen, AccountId, env, require};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    yes: u32,
    no: u32,
    yes_accounts: Vec<AccountId>,
    no_accounts: Vec<AccountId>,    
    cities: UnorderedMap<String, City>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct City {
    accounts: Vec<AccountId>,
    votes: u32
}

impl Default for Contract {
    fn default() -> Self {
        Self { yes: 0,
            no: 0,
            yes_accounts: Vec::new(),
            no_accounts: Vec::new(),
            cities: UnorderedMap::new(b"c")
         }
    }
}

#[near_bindgen]
impl Contract {

    pub fn vote_no(&mut self) {
        let voter: AccountId = env::predecessor_account_id();
        require!(!self.no_accounts.contains(&voter), "You cannot vote no twice");
        require!(!self.yes_accounts.contains(&voter), "You cannot vote both yes and no");

        self.no += 1;
        self.no_accounts.push(voter)
        
    }

    pub fn vote_yes(&mut self, city_name: String) {
        let voter: AccountId = env::predecessor_account_id();
        require!(!self.no_accounts.contains(&voter), "You cannot vote both yes and no");

        if !self.yes_accounts.contains(&voter) {
            self.yes_accounts.push(voter.clone());
            self.yes += 1; 
        }

        let city_upper: String = city_name.to_uppercase();
        
        match self.cities.get(&city_upper) {
            Some(mut city) => { 
                require!(!city.accounts.contains(&voter), "You cannot vote for the same city twice");
                city.votes += 1;
                city.accounts.push(voter);
                self.cities.insert(&city_upper, &city);
            }
            None => {
                let city: City = City{accounts: vec![voter], votes: 1};
                self.cities.insert(&city_upper, &city);
            }
        }

    }

    pub fn get_cities(&self) -> Vec<(String, City)> {
        let mut cities_list: Vec<(String, City)> = Vec::new();

        for (key, value) in self.cities.iter() {
            cities_list.push((key, value));
        }

        return cities_list
    }

    pub fn get_no(&self) -> u32 {
        return self.no
    }

    pub fn get_yes(&self) -> u32 {
        return self.yes
    }

}