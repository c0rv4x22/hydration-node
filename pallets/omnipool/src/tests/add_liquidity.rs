use super::*;
use frame_support::assert_noop;

#[test]
fn add_liquidity_works() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			let token_amount = 2000 * ONE;
			let token_price = FixedU128::from_float(0.65);

			assert_ok!(Omnipool::add_token(
				Origin::signed(LP2),
				1_000,
				token_amount,
				FixedU128::from_float(0.65)
			));

			assert_balance!(Omnipool::protocol_account(), 1_000, token_amount);

			assert_pool_state!(11_800 * ONE, 23_600 * ONE, SimpleImbalance::default());

			let liq_added = 400 * ONE;
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), 1_000, liq_added));

			assert_asset_state!(
				1_000,
				AssetState {
					reserve: token_amount + liq_added,
					hub_reserve: 1560 * ONE,
					shares: 2400 * ONE,
					protocol_shares: 2000 * ONE,
					tvl: 3120 * ONE,
					tradable: Tradable::default(),
				}
			);

			let position = Positions::<Test>::get(1).unwrap();

			let expected = Position::<Balance, AssetId> {
				asset_id: 1_000,
				amount: liq_added,
				shares: liq_added,
				price: token_price.into_inner(),
			};

			assert_eq!(position, expected);

			assert_pool_state!(12_060 * ONE, 24_120 * ONE, SimpleImbalance::default());

			assert_balance!(LP1, 1_000, 4600 * ONE);

			let minted_position = POSITIONS.with(|v| v.borrow().get(&1).copied());

			assert_eq!(minted_position, Some(LP1));
		});
}

#[test]
fn add_stable_asset_liquidity_works() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, DAI, 5000 * ONE))
		.add_endowed_accounts((LP2, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			let liq_added = 400 * ONE;
			let position_id = <PositionInstanceSequencer<Test>>::get();
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), DAI, liq_added));

			assert_asset_state!(
				DAI,
				AssetState {
					reserve: 1000 * ONE + liq_added,
					hub_reserve: 700000000000000,
					shares: 1400000000000000,
					protocol_shares: 1000 * ONE,
					tvl: 1400000000000000,
					tradable: Tradable::default(),
				}
			);

			let position = Positions::<Test>::get(position_id).unwrap();

			let expected = Position::<Balance, AssetId> {
				asset_id: DAI,
				amount: liq_added,
				shares: liq_added,
				price: FixedU128::from_float(0.5).into_inner(),
			};

			assert_eq!(position, expected);

			assert_pool_state!(10_700 * ONE, 21_400 * ONE, SimpleImbalance::default());

			assert_balance!(LP1, DAI, 4600 * ONE);

			let minted_position = POSITIONS.with(|v| v.borrow().get(&position_id).copied());

			assert_eq!(minted_position, Some(LP1));
		});
}

#[test]
fn add_liquidity_for_non_pool_token_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, 2000 * ONE,),
				Error::<Test>::AssetNotFound
			);
		});
}

#[test]
fn add_liquidity_with_insufficient_balance_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::add_token(
				Origin::signed(LP1),
				1_000,
				2000 * ONE,
				FixedU128::from_float(0.65)
			));

			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP3), 1_000, 2000 * ONE,),
				Error::<Test>::InsufficientBalance
			);
		});
}

#[test]
fn add_liquidity_exceeding_weight_cap_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_asset_weight_cap((1, 100))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::add_token(
				Origin::signed(LP1),
				1_000,
				100 * ONE,
				FixedU128::from_float(0.65)
			));

			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP1), 1_000, 2000 * ONE,),
				Error::<Test>::AssetWeightCapExceeded
			);
		});
}

#[test]
fn add_insufficient_liquidity_fails() {
	ExtBuilder::default()
		.add_endowed_accounts((LP1, 1_000, 5000 * ONE))
		.with_min_added_liquidity(5 * ONE)
		.with_asset_weight_cap((1, 100))
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::add_token(
				Origin::signed(LP1),
				1_000,
				2000 * ONE,
				FixedU128::from_float(0.65)
			));

			assert_noop!(
				Omnipool::add_liquidity(Origin::signed(LP3), 1_000, ONE,),
				Error::<Test>::InsufficientLiquidity
			);
		});
}
