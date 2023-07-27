use near_sdk::json_types::U128;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};
use nft_auction_contract::NFTAuctionContract;
use near_sdk::ONE_NEAR;
#[test]
fn test_mint_nft() {
    let context = VMContextBuilder::new()
        .current_account_id(accounts(0))
        .build();
    testing_env!(context.clone());

    let mut contract = NFTAuctionContract::new(accounts(0));
    let token_id = "token1".to_string();
    assert_eq!(contract.mint_nft(token_id.clone(), accounts(0)), true);
}

#[test]
fn test_start_auction() {
    let context = VMContextBuilder::new()
        .current_account_id(accounts(0))
        .build();
    testing_env!(context.clone());

    let mut contract = NFTAuctionContract::new(accounts(0));
    let token_id = "token1".to_string();
    assert_eq!(contract.mint_nft(token_id.clone(), accounts(0)), true);

    // Starting auction for NFT
    let nft_index = 0;
    let start_bid = U128::from(ONE_NEAR);
    let duration = 1000; // Some duration in blocks (change as needed)

    assert_eq!(
        contract.start_auction(nft_index, start_bid, duration),
        true
    );
}

#[test]
fn test_place_bid() {
    let context = VMContextBuilder::new()
        .current_account_id(accounts(0))
        .build();
    testing_env!(context.clone());

    let mut contract = NFTAuctionContract::new(accounts(0));
    let token_id = "token1".to_string();
    assert_eq!(contract.mint_nft(token_id.clone(), accounts(0)), true);

    // Starting auction for NFT
    let nft_index = 0;
    let start_bid = U128::from(ONE_NEAR);
    let duration = 1000; // Some duration in blocks (change as needed)

    assert_eq!(
        contract.start_auction(nft_index, start_bid, duration),
        true
    );

    // Placing a bid on the auction
    let bid_amount = U128::from(2 * ONE_NEAR); // Place a bid greater than the start_bid
    assert_eq!(contract.place_bid(0, bid_amount), true);
}

#[test]
fn test_end_auction() {
    let context = VMContextBuilder::new()
        .current_account_id(accounts(0))
        .build();
    testing_env!(context.clone());

    let mut contract = NFTAuctionContract::new(accounts(0));
    let token_id = "token1".to_string();
    assert_eq!(contract.mint_nft(token_id.clone(), accounts(0)), true);

    // Starting auction for NFT
    let nft_index = 0;
    let start_bid = U128::from(ONE_NEAR);
    let duration = 1000; // Some duration in blocks (change as needed)

    assert_eq!(
        contract.start_auction(nft_index, start_bid, duration),
        true
    );

    // Placing a bid on the auction
    let bid_amount = U128::from(2 * ONE_NEAR); // Place a bid greater than the start_bid
    assert_eq!(contract.place_bid(0, bid_amount), true);

    // Ending the auction
    assert_eq!(contract.end_auction(0), true);
}
