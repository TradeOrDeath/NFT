#![allow(unused_imports)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Balance, Promise};

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[ext_contract(ext_nft)]
pub trait ExtNFT {
    fn nft_mint(&mut self, token_id: String, owner_id: AccountId) -> bool;
}

#[near_bindgen]
#[derive( Default,BorshDeserialize, BorshSerialize)]
pub struct NFTAuctionContract {
    owner_id: AccountId,
    nft_tokens: Vec<NFT>,
    auctions: Vec<Auction>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct NFT {
    token_id: String,
    owner_id: AccountId,
    // Add more NFT properties as needed
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Auction {
    nft_index: u64,
    start_bid: Balance,
    end_time: u64,
    highest_bid: Balance,
    highest_bidder: Option<AccountId>,
}

#[near_bindgen]
impl NFTAuctionContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            owner_id,
            nft_tokens: vec![],
            auctions: vec![],
        }
    }

    pub fn mint_nft(&mut self, token_id: String, owner_id: AccountId) -> bool {
    assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can mint NFTs");
    let nft = NFT { token_id, owner_id };
    self.nft_tokens.push(nft);
    true
}


    pub fn start_auction(&mut self, nft_index: u64, start_bid: U128, duration: u64) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can start an auction"
        );

        let nft = self.nft_tokens.get(nft_index as usize).expect("NFT not found");
        assert_eq!(nft.owner_id, env::predecessor_account_id(), "Only NFT owner can start an auction");

        let start_bid: Balance = start_bid.into();
        assert!(
            start_bid >= ONE_NEAR,
            "Start bid must be greater than or equal to 1 NEAR"
        );

        let end_time = env::block_timestamp() + duration;
        let auction = Auction {
            nft_index,
            start_bid,
            end_time,
            highest_bid: 0,
            highest_bidder: None,
        };
        self.auctions.push(auction);
        true
    }

    pub fn place_bid(&mut self, auction_index: u64, bid: U128) -> bool {
        let bid: Balance = bid.into();
        assert!(
            bid >= ONE_NEAR,
            "Bid amount must be greater than or equal to 1 NEAR"
        );

        let mut auction = self.auctions.get_mut(auction_index as usize).expect("Auction not found");
        assert!(
            env::block_timestamp() <= auction.end_time,
            "Auction has ended"
        );

        if bid > auction.highest_bid {
            auction.highest_bid = bid;
            auction.highest_bidder = Some(env::predecessor_account_id());
        }

        true
    }

    pub fn end_auction(&mut self, auction_index: u64) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can end an auction"
        );

        let mut auction = self.auctions.get_mut(auction_index as usize).expect("Auction not found");
        assert!(
            env::block_timestamp() > auction.end_time,
            "Auction has not ended yet"
        );

        let highest_bidder = auction.highest_bidder.take();
        if let Some(bidder) = highest_bidder {
            let nft = self.nft_tokens.get_mut(auction.nft_index as usize).expect("NFT not found");
            nft.owner_id = bidder.clone();

            // Transfer the highest bid amount to the NFT owner
            Promise::new(bidder).transfer(auction.highest_bid);
        }

        // Remove the auction from the list
        self.auctions.remove(auction_index as usize);

        true
    }
}
