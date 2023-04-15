pub mod passing {
    use crate::utils::{
        active_auctions_of_author, auction, auction_count, auctions_of_author, auctions_won, bid,
        create_auction, get_contract_instance,
    };
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    async fn can_bid() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        bid(&instance, 1, 400).await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            vec![1]
        );
    }

    #[tokio::test]
    async fn auction_count_increases() {
        let (instance, wallet) = get_contract_instance().await;

        // retrieving active_auctions_of_author and auctions_of_author and auction_count before
        let auction_count_before = auction_count(&instance).await;
        let active_auctions_of_author_before =
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into())).await;
        let auctions_of_author_before =
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, add 3 txs above, and then reaches 5 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        // testing that active_auctions_of_author and auctions_of_author and auction_count are all the increased
        assert_eq!(auction_count(&instance).await, auction_count_before + 1);
        assert_eq!(
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into()))
                .await
                .len(),
            active_auctions_of_author_before.len() + 1
        );
        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into()))
                .await
                .len(),
            auctions_of_author_before.len() + 1
        );
    }

    #[tokio::test]
    async fn auction_details_are_stored() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        let auction = auction(&instance, 1).await;

        assert_eq!(auction.asset_id, ContractId::zeroed());
        assert_eq!(auction.author, Identity::Address(wallet.address().into()));
        assert_eq!(
            auction.beneficiary,
            Identity::Address(wallet.address().into())
        );
        assert_eq!(auction.ended, false);
        assert_eq!(auction.end_time, 5);
        assert_eq!(auction.opening_price, 400);
        assert_eq!(auction.reserve_price, 100);
        assert_eq!(auction.start_time, 2);
        assert_eq!(auction.winner, None);
    }
}

mod failing {
    use crate::utils::{create_auction, get_contract_instance};
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    #[should_panic(expected = "EndPriceCannotBeLargerThanStartPrice")]
    async fn end_price_cannot_be_larger_than_start_price() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            100,
            400,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionCannotEndInThePast")]
    async fn auction_cannot_end_in_the_past() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            1,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionCannotStartInThePast")]
    async fn auction_cannot_start_in_the_past() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            1, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionCannotEndBeforeItStarts")]
    async fn auction_cannot_end_before_it_starts() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            10, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;
    }
}