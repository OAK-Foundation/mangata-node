//! # XYK pallet

//! Provides functions for token operations, swapping tokens, creating token pools, minting and burning liquidity and supporting public functions
//!
//! ### Token operation functions:
//! - create_pool
//! - mint_liquidity
//! - burn_liquidity
//! - sell_asset
//! - buy_asset
//!
//! ### Supporting public functions:
//! - calculate_sell_price
//! - calculate_buy_price
//! - calculate_sell_price_id
//! - calculate_buy_price_id
//! - get_liquidity_token
//! - get_burn_amount
//! - account_id
//! - settle_treasury_buy_and_burn
//!
//! # fn create_pool
//! -Sets the initial ratio/price of both assets to each other depending on amounts of each assets when creating pool.
//!
//! -Transfers assets from user to vault and makes appropriate entry to pools map, where are assets amounts kept.
//!
//! -Issues new liquidity asset in amount corresponding to amounts creating the pool, marks them as liquidity assets corresponding to this pool and transfers them to user.
//! first_token_amount
//! ### arguments
//! `origin` - sender of a fn, user creating the pool
//!
//! `first_token_id` - id of first token which will be directly inter-tradeable in a pair of first_token_id-second_token_id
//!
//! `first_token_amount` - amount of first token in which the pool will be initiated, which will set their initial ratio/price
//!
//! `second_token_id` - id of second token which will be directly inter-tradeable in a pair of first_token_id-second_token_id
//!
//! `second_token_amount` - amount of second token in which the pool will be initiated, which will set their initial ratio/price
//!
//! ### Example
//! ```
//! create_pool(
//!    Origin::signed(1),
//!    0,
//!    1000,
//!    1,
//!    2000,
//! )
//! ```
//! Account_id 1 created pool with tokens 0 and 1, with amounts 1000, 2000. Initial ratio is 1:2. Liquidity token with new id created in an amount of 3000 and transfered to user 1.
//!
//! ### Errors
//! `ZeroAmount` - creating pool with 0 amount of first or second token
//!
//! `PoolAlreadyExists` - creating pool which already exists
//!
//! `NotEnoughTokens` - creating pool with amounts higher then user owns
//!
//! `SameToken` - creating pool with same token
//!
//! # fn sell_token
//! -Sells/exchanges set amount of sold token for corresponding amount by xyk formula of bought token
//! ### arguments
//! `origin` - sender of a fn, user creating the pool
//!
//! `sold_token_id` - token which will be sold
//!
//! `bought_token_id` - token which will be bought
//!
//! `sold_token_amount` - amount of token to be sold
//!
//! `min_amount_out` - minimal acceptable amount of bought token received after swap
//!
//! ### Example
//! ```
//! sell_token (
//!    Origin::signed(1),
//!    0,
//!    1,
//!    1000,
//!    800,
//!)
//! ```
//! Account_id 1 sells/exchanges 1000 token 0 for corresponding amount of token 1, while requiring at least 800 token 1
//!
//! ### Errors
//! `ZeroAmount` - buying 0 tokens
//!
//! `NoSuchPool` - pool sold_token_id - bought_token_id does not exist
//!
//! `NotEnoughTokens` - selling more tokens then user owns
//!
//! `InsufficientOutputAmount` - bought tokens to receive amount is lower then required min_amount_out
//!
//! # fn buy_token
//! -Buys/exchanges set amount of bought token for corresponding amount by xyk formula of sold token
//! ### arguments
//! `origin` - sender of a fn, user creating the pool
//!
//! `sold_token_id` - token which will be sold
//!
//! `bought_token_id` - token which will be bought
//!
//! `bought_token_amount` - amount of token to be bought
//!
//! `max_amount_in` - maximal acceptable amount of sold token to pay for requested bought amount
//!
//! ### Example
//! ```
//! buy_token (
//!    Origin::signed(1),
//!    0,
//!    1,
//!    1000,
//!    800,
//!)
//! ```
//! Account_id 1 buys/exchanges 1000 tokens 1 by paying corresponding amount by xyk formula of tokens 0
//!
//! ### Errors
//! `ZeroAmount` - selling 0 tokens
//!
//! `NoSuchPool` - pool sold_token_id - bought_token_id does not exist
//!
//! `NotEnoughTokens` - selling more tokens then user owns
//!
//! `InsufficientInputAmount` - sold tokens to pay is higher then maximum acceptable value of max_amount_in
//!
//! # fn mint_liquidity
//! -Adds liquidity to pool, providing both tokens in actual ratio
//! -First token amount is provided by user, second token amount is calculated by function, depending on actual ratio
//! -Mints and transfers corresponding amount of liquidity token to mintin user
//!
//! ### arguments
//! `origin` - sender of a fn, user creating the pool
//!
//! first_token_id - first token in pair
//!
//! second_token_id - second token in pair
//!
//! first_token_amount - amount of first_token_id, second token amount will be calculated
//!
//! ### Example
//! ```
//! mint_liquidity (
//!    Origin::signed(1),
//!    0,
//!    1,
//!    1000,
//!)
//! ```
//! If pool token 0 - token 1 has tokens in amounts 9000:18000 (total liquidity tokens 27000)
//!
//! Account_id 1 added liquidity to pool token 0 - token 1, by providing 1000 token 0 and corresponding amount of token 1. In this case 2000, as the ratio in pool is 1:2.
//! Account_id 1 also receives corresponding liquidity tokens in corresponding amount. In this case he gets 10% of all corresponding liquidity tokens, as he is providing 10% of all provided liquidity in pool.
//! 3000 out of total 30000 liquidity tokens is now owned by Account_id 1
//!
//! ### Errors
//! `ZeroAmount` - minting with 0 tokens
//!
//! `NoSuchPool` - pool first_token_id - second_token_id does not exist
//!
//! `NotEnoughTokens` -  minting with more tokens then user owns, either first_token_id or second_token_id
//!     
//! # fn burn_liquidity
//! -Removes tokens from liquidity pool and transfers them to user, by burning user owned liquidity tokens
//! -Amount of tokens is determined by their ratio in pool and amount of liq tokens burned
//!
//! ### arguments
//! `origin` - sender of a fn, user creating the pool
//!
//! first_token_id - first token in pair
//!
//! second_token_id - second token in pair
//!
//! liquidity_token_amount - amount of liquidity token amount to burn
//!
//! ### Example
//! ```
//! burn_liquidity (
//!    Origin::signed(1),
//!    0,
//!    1,
//!    3000,
//!)
//! ```
//! If pool token 0 - token 1 has tokens in amounts 10000:20000 (total liquidity tokens 30000)
//!
//! Account_id 1 is burning 3000 liquidity tokens of pool token 0 - token 1
//! As Account_id 1 is burning 10% of total liquidity tokens for this pool, user receives in this case 1000 token 0 and 2000 token 1
//!
//! ### Errors
//! `ZeroAmount` - burning 0 liquidity tokens
//!
//! `NoSuchPool` - pool first_token_id - second_token_id does not exist
//!
//! `NotEnoughTokens` -  burning more liquidity tokens than user owns
//!
//! # calculate_sell_price
//! - Supporting public function accessible through rpc call which calculates and returns bought_token_amount while providing sold_token_amount and respective reserves
//! # calculate_buy_price
//! - Supporting public function accessible through rpc call which calculates and returns sold_token_amount while providing bought_token_amount and respective reserves
//! # calculate_sell_price_id
//! - Same as calculate_sell_price, but providing token_id instead of reserves. Reserves are fetched by function.
//! # calculate_buy_price_id
//! - Same as calculate_buy_price, but providing token_id instead of reserves. Reserves are fetched by function.
//! # get_liquidity_token
//! - Supporting public function accessible through rpc call which returns liquidity_token_id while providing pair token ids
//! # get_burn_amount
//! - Supporting public function accessible through rpc call which returns amounts of tokens received by burning provided liquidity_token_amount in pool of provided token ids
//! # account_id
//! - Returns palled account_id
//! # settle_treasury_buy_and_burn
//! - Supporting function which takes tokens to alocate to treasury and tokens to be used to burn mangata
//! - First step is deciding whether we are using sold or bought token id, depending which is closer to mangata token
//! - In second step, if tokens are mangata, they are placed to treasury and removed from corresponding pool. If tokens are not mangata, but are available in mangata pool,
//!   they are swapped to mangata and placed to treasury and removed from corresponding pool. If token is not connected to mangata, token is temporarily placed to treasury and burn treasury.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{DispatchError, DispatchResult},
    ensure,
    sp_runtime::ModuleId,
    weights::Pays,
    StorageMap,
};
use frame_system::ensure_signed;
use sp_core::U256;
// TODO documentation!
use codec::FullCodec;
use frame_support::sp_runtime::traits::AccountIdConversion;
use frame_support::traits::{ExistenceRequirement, Get, WithdrawReasons};
use frame_support::Parameter;
use mangata_primitives::{Balance, TokenId};
use orml_tokens::{MultiTokenCurrency, MultiTokenCurrencyExtended};
use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member};
use sp_runtime::traits::{SaturatedConversion, Zero};
use sp_std::fmt::Debug;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: MultiTokenCurrencyExtended<Self::AccountId>;
    type NativeCurrencyId: Get<TokenId>;
}

const PALLET_ID: ModuleId = ModuleId(*b"79b14c96");
// 1/100 %
const TREASURY_PERCENTAGE: u128 = 5;
const BUYANDBURN_PERCENTAGE: u128 = 5;
const SWAPFEE_PERCENTAGE: u128 = 30;
const MANGATA_ID: u128 = 0;

decl_error! {
    /// Errors
    pub enum Error for Module<T: Trait> {
        VaultAlreadySet,
        PoolAlreadyExists,
        NotEnoughAssets,
        NoSuchPool,
        NoSuchLiquidityAsset,
        NotEnoughReserve,
        ZeroAmount,
        InsufficientInputAmount,
        InsufficientOutputAmount,
        SameAsset,
        AssetAlreadyExists,
        AssetDoesNotExists,
        DivisionByZero,
        NotMangataLiquidityAsset,
        SecondAssetAmountExceededExpectations,
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        //TODO add trading events
        PoolCreated(AccountId, TokenId, Balance, TokenId, Balance),
        AssetsSwapped(AccountId, TokenId, Balance, TokenId, Balance),
        LiquidityMinted(
            AccountId,
            TokenId,
            Balance,
            TokenId,
            Balance,
            TokenId,
            Balance,
        ),
        LiquidityBurned(
            AccountId,
            TokenId,
            Balance,
            TokenId,
            Balance,
            TokenId,
            Balance,
        ),
    }
);

// XYK exchange pallet storage.
decl_storage! {
    trait Store for Module<T: Trait> as XykStorage {

        Pools get(fn asset_pool): map hasher(opaque_blake2_256) (TokenId, TokenId) => Balance;

        LiquidityAssets get(fn liquidity_asset): map hasher(opaque_blake2_256) (TokenId, TokenId) => TokenId;
        LiquidityPools get(fn liquidity_pool): map hasher(opaque_blake2_256) TokenId => (TokenId, TokenId);
        Treasury get(fn treasury): map hasher(opaque_blake2_256) TokenId => Balance;
        TreasuryBurn get(fn treasury_burn): map hasher(opaque_blake2_256) TokenId => Balance;

        Nonce get (fn nonce): u32;

    }
    add_extra_genesis {
        config(created_pools_for_staking): Vec<(T::AccountId, TokenId, Balance, TokenId, Balance, TokenId)>;

        build(|config: &GenesisConfig<T>| {
            config.created_pools_for_staking.iter().for_each(|(account_id, native_token_id, native_token_amount, pooled_token_id, pooled_token_amount, liquidity_token_id)| {
                if T::Currency::exists({*liquidity_token_id}.into()){
                    assert!(<Module<T>>::mint_liquidity( T::Origin::from(Some(account_id.clone()).into()), *native_token_id, *pooled_token_id, *native_token_amount, Some(*pooled_token_amount)).is_ok(), "Pool mint failed");
                }
                else{
                    let created_liquidity_token_id: TokenId = T::Currency::get_next_currency_id().into();
                    assert!(created_liquidity_token_id == *liquidity_token_id, "Assets not initialized in the expected sequence");
                    assert!(<Module<T>>::create_pool( T::Origin::from(Some(account_id.clone()).into()), *native_token_id, *native_token_amount, *pooled_token_id, *pooled_token_amount).is_ok(), "Pool creation failed");
                }
            })
        })
    }
}

// XYK extrinsics.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_pool(
            origin,
            first_asset_id: TokenId,
            first_asset_amount: Balance,
            second_asset_id: TokenId,
            second_asset_amount: Balance
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;

            <Self as XykFunctionsTrait<T::AccountId>>::create_pool(sender, first_asset_id.into(), first_asset_amount.into(), second_asset_id.into(), second_asset_amount.into())

        }

        // you will sell your sold_asset_amount of sold_asset_id to get some amount of bought_asset_id
        #[weight = (10_000, Pays::No)]
        pub fn sell_asset (
            origin,
            sold_asset_id: TokenId,
            bought_asset_id: TokenId,
            sold_asset_amount: Balance,
            min_amount_out: Balance,
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;
            <Self as XykFunctionsTrait<T::AccountId>>::sell_asset(sender, sold_asset_id.into(), bought_asset_id.into(), sold_asset_amount.into(), min_amount_out.into())

        }

        #[weight = (10_000, Pays::No)]
        pub fn buy_asset (
            origin,
            sold_asset_id: TokenId,
            bought_asset_id: TokenId,
            bought_asset_amount: Balance,
            max_amount_in: Balance,
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;

            <Self as XykFunctionsTrait<T::AccountId>>::buy_asset(sender, sold_asset_id.into(), bought_asset_id.into(), bought_asset_amount.into(), max_amount_in.into())

        }

        #[weight = 10_000]
        pub fn mint_liquidity (
            origin,
            first_asset_id: TokenId,
            second_asset_id: TokenId,
            first_asset_amount: Balance,
            expected_second_asset_amount: Option<Balance>,
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;

            <Self as XykFunctionsTrait<T::AccountId>>::mint_liquidity(sender, first_asset_id.into(), second_asset_id.into(), first_asset_amount.into(), expected_second_asset_amount.into())

        }

        #[weight = 10_000]
        pub fn burn_liquidity (
            origin,
            first_asset_id: TokenId,
            second_asset_id: TokenId,
            liquidity_asset_amount: Balance,
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;

            <Self as XykFunctionsTrait<T::AccountId>>::burn_liquidity(sender, first_asset_id.into(), second_asset_id.into(), liquidity_asset_amount.into())

        }
    }
}

impl<T: Trait> Module<T> {
    // Callculate amount of tokens to be bought by sellling sell_amount
    pub fn calculate_sell_price(
        input_reserve: Balance,
        output_reserve: Balance,
        sell_amount: Balance,
    ) -> Result<Balance, DispatchError> {
        let input_reserve_saturated: U256 = input_reserve.saturated_into::<u128>().into();
        let output_reserve_saturated: U256 = output_reserve.saturated_into::<u128>().into();
        let sell_amount_saturated: U256 = sell_amount.saturated_into::<u128>().into();

        let input_amount_with_fee: U256 = sell_amount_saturated * 997;
        let numerator: U256 = input_amount_with_fee * output_reserve_saturated;
        let denominator: U256 = input_reserve_saturated * 1000 + input_amount_with_fee;
        let result = numerator
            .checked_div(denominator)
            .ok_or_else(|| DispatchError::from(Error::<T>::DivisionByZero))?;
        Ok(result.saturated_into::<u128>().saturated_into::<Balance>())
    }

    pub fn calculate_sell_price_no_fee(
        // Callculate amount of tokens to be received by sellling sell_amount, without fee
        input_reserve: Balance,
        output_reserve: Balance,
        sell_amount: Balance,
    ) -> Result<Balance, DispatchError> {
        let input_reserve_saturated: U256 = input_reserve.saturated_into::<u128>().into();
        let output_reserve_saturated: U256 = output_reserve.saturated_into::<u128>().into();
        let sell_amount_saturated: U256 = sell_amount.saturated_into::<u128>().into();

        let numerator: U256 = sell_amount_saturated * output_reserve_saturated;
        let denominator: U256 = input_reserve_saturated + sell_amount_saturated;
        let result = numerator
            .checked_div(denominator)
            .ok_or_else(|| DispatchError::from(Error::<T>::DivisionByZero))?;
        Ok(result.saturated_into::<u128>().saturated_into::<Balance>())
    }

    // Callculate amount of tokens to be paid, when buying buy_amount
    pub fn calculate_buy_price(
        input_reserve: Balance,
        output_reserve: Balance,
        buy_amount: Balance,
    ) -> Balance {
        let input_reserve_saturated: U256 = input_reserve.saturated_into::<u128>().into();
        let output_reserve_saturated: U256 = output_reserve.saturated_into::<u128>().into();
        let buy_amount_saturated: U256 = buy_amount.saturated_into::<u128>().into();

        let numerator: U256 = input_reserve_saturated * buy_amount_saturated * 1000;
        let denominator: U256 = (output_reserve_saturated - buy_amount_saturated) * 997;
        let result: U256 = numerator / denominator + 1;

        result.saturated_into::<u128>().saturated_into::<Balance>()
    }

    pub fn get_liquidity_asset(first_asset_id: TokenId, second_asset_id: TokenId) -> TokenId {
        if LiquidityAssets::contains_key((first_asset_id, second_asset_id)) {
            LiquidityAssets::get((first_asset_id, second_asset_id))
        } else {
            LiquidityAssets::get((second_asset_id, first_asset_id))
        }
    }

    // Calculate first and second token amounts depending on liquidity amount to burn
    pub fn get_burn_amount(
        first_asset_id: TokenId,
        second_asset_id: TokenId,
        liquidity_asset_amount: Balance,
    ) -> (Balance, Balance) {
        // Get token reserves and liquidity asset id
        let liquidity_asset_id = Self::get_liquidity_asset(first_asset_id, second_asset_id);
        let first_asset_reserve_u256: U256 = Pools::get((first_asset_id, second_asset_id))
            .saturated_into::<u128>()
            .into();
        let second_asset_reserve_u256: U256 = Pools::get((second_asset_id, first_asset_id))
            .saturated_into::<u128>()
            .into();
        let total_liquidity_assets_u256: U256 =
            T::Currency::total_issuance(liquidity_asset_id.into())
                .saturated_into::<u128>()
                .into();
        let liquidity_asset_amount_u256: U256 =
            liquidity_asset_amount.saturated_into::<u128>().into();

        // Calculate first and second token amount to be withdrawn
        let first_asset_amount_u256 =
            first_asset_reserve_u256 * liquidity_asset_amount_u256 / total_liquidity_assets_u256;
        let second_asset_amount_u256 =
            second_asset_reserve_u256 * liquidity_asset_amount_u256 / total_liquidity_assets_u256;

        // Conversion to Balance
        let second_asset_amount = second_asset_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Balance>();
        let first_asset_amount = first_asset_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Balance>();

        (first_asset_amount, second_asset_amount)
    }

    //TODO U256?
    fn settle_treasury_and_burn(
        sold_asset_id: TokenId,
        bought_asset_id: TokenId,
        sold_asset_amount: Balance,
    ) -> DispatchResult {
        let vault = Self::account_id();
        let mangata_id = MANGATA_ID.saturated_into::<TokenId>();

        // Getting token reserves
        let input_reserve = Pools::get((sold_asset_id, bought_asset_id));
        let output_reserve = Pools::get((bought_asset_id, sold_asset_id));

        // Setting initial settling token id, treasury and burn amount
        let mut settling_asset_id = bought_asset_id;
        let mut treasury_amount = sold_asset_amount
            * TREASURY_PERCENTAGE.saturated_into::<Balance>()
            / 10000.saturated_into::<Balance>();
        let mut burn_amount = sold_asset_amount * BUYANDBURN_PERCENTAGE.saturated_into::<Balance>()
            / 10000.saturated_into::<Balance>();

        // Check whether to settle treasury and buyburn with sold or bought asset.
        // If sold token is directly mangata, or is in pair with mangata and bought id is not and bought token is not mangata, we use sold token as settling token
        if sold_asset_id == mangata_id
            || (Pools::contains_key((sold_asset_id, mangata_id))
                && !Pools::contains_key((bought_asset_id, mangata_id))
                && bought_asset_id != mangata_id)
        {
            settling_asset_id = sold_asset_id;

            // Removing settling amount from pool
            Pools::insert(
                (&sold_asset_id, &bought_asset_id),
                input_reserve - burn_amount - treasury_amount,
            );
        }
        // Bought token is used as settling token in rest of the cases
        else {
            // Sold amount recalculated to bought asset amount
            treasury_amount = treasury_amount * output_reserve / input_reserve;
            burn_amount = burn_amount * output_reserve / input_reserve;

            // Removing settling amount from pool
            Pools::insert(
                (&bought_asset_id, &sold_asset_id),
                output_reserve - treasury_amount - burn_amount,
            );
        }

        // If settling token is mangata, treasury amount is added to treasury and burn amount is burned from corresponding pool
        if settling_asset_id == mangata_id {
            // Mangata insert to treasury
            Treasury::insert(mangata_id, Treasury::get(mangata_id) + treasury_amount);

            // Mangata burned from pool
            T::Currency::burn_and_settle(mangata_id.into(), &vault, burn_amount.into())?;
        }
        //If settling token is connected to mangata, token is swapped in corresponding pool to mangata without fee
        else if Pools::contains_key((settling_asset_id, mangata_id)) {
            // Getting token reserves
            let input_reserve = Pools::get((settling_asset_id, mangata_id));
            let output_reserve = Pools::get((mangata_id, settling_asset_id));

            // Calculating swapped mangata amount
            let treasury_amount_in_mangata =
                Self::calculate_sell_price_no_fee(input_reserve, output_reserve, treasury_amount)?;
            let burn_amount_in_mangata =
                Self::calculate_sell_price_no_fee(input_reserve, output_reserve, burn_amount)?;

            // Apply changes in token pools, adding treasury and burn amounts of settling token, removing  treasury and burn amounts of mangata
            Pools::insert(
                (settling_asset_id, mangata_id),
                input_reserve + treasury_amount + burn_amount,
            );
            Pools::insert(
                (mangata_id, settling_asset_id),
                output_reserve - treasury_amount_in_mangata - burn_amount_in_mangata,
            );

            // Mangata insert to treasury
            Treasury::insert(
                mangata_id,
                Treasury::get(mangata_id) + treasury_amount_in_mangata,
            );

            // Mangata burned from pool
            T::Currency::burn_and_settle(mangata_id.into(), &vault, burn_amount_in_mangata.into())?;
        }
        // Settling token has no mangata connection, settling token is added to treasuries
        else {
            // Settling token insert to treasury
            Treasury::insert(
                settling_asset_id,
                Treasury::get(settling_asset_id) + treasury_amount,
            );
            // Settling token insert to treasury for later burn
            TreasuryBurn::insert(
                settling_asset_id,
                TreasuryBurn::get(settling_asset_id) + burn_amount,
            );
        }
        Ok(())
    }

    fn account_id() -> T::AccountId {
        PALLET_ID.into_account()
    }
}

pub trait XykFunctionsTrait<AccountId> {
    type Balance: AtLeast32BitUnsigned
        + FullCodec
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Default
        + From<Balance>
        + Into<Balance>;

    type CurrencyId: Parameter
        + Member
        + Copy
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + AtLeast32BitUnsigned
        + FullCodec
        + From<TokenId>
        + Into<TokenId>;

    fn create_pool(
        sender: AccountId,
        first_asset_id: Self::CurrencyId,
        first_asset_amount: Self::Balance,
        second_asset_id: Self::CurrencyId,
        second_asset_amount: Self::Balance,
    ) -> DispatchResult;

    fn sell_asset(
        sender: AccountId,
        sold_asset_id: Self::CurrencyId,
        bought_asset_id: Self::CurrencyId,
        sold_asset_amount: Self::Balance,
        min_amount_out: Self::Balance,
    ) -> DispatchResult;

    fn buy_asset(
        sender: AccountId,
        sold_asset_id: Self::CurrencyId,
        bought_asset_id: Self::CurrencyId,
        bought_asset_amount: Self::Balance,
        max_amount_in: Self::Balance,
    ) -> DispatchResult;

    fn mint_liquidity(
        sender: AccountId,
        first_asset_id: Self::CurrencyId,
        second_asset_id: Self::CurrencyId,
        first_asset_amount: Self::Balance,
        expected_second_asset_amount: Option<Self::Balance>,
    ) -> DispatchResult;

    fn burn_liquidity(
        sender: AccountId,
        first_asset_id: Self::CurrencyId,
        second_asset_id: Self::CurrencyId,
        liquidity_asset_amount: Self::Balance,
    ) -> DispatchResult;

    fn get_tokens_required_for_minting(
        liquidity_asset_id: Self::CurrencyId,
        liquidity_token_amount: Self::Balance,
    ) -> (
        Self::CurrencyId,
        Self::Balance,
        Self::CurrencyId,
        Self::Balance,
    );
}

impl<T: Trait> XykFunctionsTrait<T::AccountId> for Module<T> {
    type Balance = Balance;

    type CurrencyId = TokenId;

    fn create_pool(
        sender: T::AccountId,
        first_asset_id: Self::CurrencyId,
        first_asset_amount: Self::Balance,
        second_asset_id: Self::CurrencyId,
        second_asset_amount: Self::Balance,
    ) -> DispatchResult {
        let vault: T::AccountId = Module::<T>::account_id();

        // Ensure pool is not created with zero amount
        ensure!(
            !first_asset_amount.is_zero() && !second_asset_amount.is_zero(),
            Error::<T>::ZeroAmount,
        );

        // Ensure pool does not exists yet
        ensure!(
            !Pools::contains_key((first_asset_id, second_asset_id)),
            Error::<T>::PoolAlreadyExists,
        );

        // Ensure pool does not exists yet
        ensure!(
            !Pools::contains_key((second_asset_id, first_asset_id)),
            Error::<T>::PoolAlreadyExists,
        );

        // Getting users token balances
        let first_asset_free_balance: Self::Balance =
            T::Currency::free_balance(first_asset_id.into(), &sender).into();
        let second_asset_free_balance: Self::Balance =
            T::Currency::free_balance(second_asset_id.into(), &sender).into();

        // Ensure user has enough withdrawable tokens to create pool in amounts required
        ensure!(
            first_asset_free_balance >= first_asset_amount,
            Error::<T>::NotEnoughAssets,
        );

        ensure!(
            second_asset_free_balance >= second_asset_amount,
            Error::<T>::NotEnoughAssets,
        );

        T::Currency::ensure_can_withdraw(
            first_asset_id.into(),
            &sender,
            first_asset_amount.into(),
            WithdrawReasons::all(),
            { first_asset_free_balance - first_asset_amount }.into(),
        )
        .or(Err(Error::<T>::NotEnoughAssets))?;

        T::Currency::ensure_can_withdraw(
            second_asset_id.into(),
            &sender,
            second_asset_amount.into(),
            WithdrawReasons::all(),
            { second_asset_free_balance - second_asset_amount }.into(),
        )
        .or(Err(Error::<T>::NotEnoughAssets))?;

        // Ensure pool is not created with same token in pair
        ensure!(first_asset_id != second_asset_id, Error::<T>::SameAsset,);

        // Liquidity token amount calculation
        let initial_liquidity = first_asset_amount + second_asset_amount;

        Pools::insert((first_asset_id, second_asset_id), first_asset_amount);

        Pools::insert((second_asset_id, first_asset_id), second_asset_amount);

        // Moving tokens from user to vault
        T::Currency::transfer(
            first_asset_id.into(),
            &sender,
            &vault,
            first_asset_amount.into(),
            ExistenceRequirement::AllowDeath,
        )?;

        T::Currency::transfer(
            second_asset_id.into(),
            &sender,
            &vault,
            second_asset_amount.into(),
            ExistenceRequirement::AllowDeath,
        )?;

        // Creating new liquidity token and transfering it to user
        let liquidity_asset_id: Self::CurrencyId =
            T::Currency::create(&sender, initial_liquidity.into()).into();

        // Adding info about liquidity asset
        LiquidityAssets::insert((first_asset_id, second_asset_id), liquidity_asset_id);
        LiquidityPools::insert(liquidity_asset_id, (first_asset_id, second_asset_id));

        Module::<T>::deposit_event(RawEvent::PoolCreated(
            sender,
            first_asset_id,
            first_asset_amount,
            second_asset_id,
            second_asset_amount,
        ));

        Ok(())
    }

    fn sell_asset(
        sender: T::AccountId,
        sold_asset_id: Self::CurrencyId,
        bought_asset_id: Self::CurrencyId,
        sold_asset_amount: Self::Balance,
        min_amount_out: Self::Balance,
    ) -> DispatchResult {
        // Ensure pool exists
        ensure!(
            Pools::contains_key((sold_asset_id, bought_asset_id)),
            Error::<T>::NoSuchPool,
        );

        // Ensure not selling zero amount
        ensure!(!sold_asset_amount.is_zero(), Error::<T>::ZeroAmount,);

        // Get token reserves
        let input_reserve = Pools::get((sold_asset_id, bought_asset_id));
        let output_reserve = Pools::get((bought_asset_id, sold_asset_id));
        // Calculate bought asset amount to be received by paying sold asset amount
        let bought_asset_amount =
            Module::<T>::calculate_sell_price(input_reserve, output_reserve, sold_asset_amount)?;

        // Ensure user has enough tokens to sell
        ensure!(
            T::Currency::free_balance(sold_asset_id.into(), &sender).into() >= sold_asset_amount,
            Error::<T>::NotEnoughAssets,
        );

        // Ensure bought token amount is higher then requested minimal amount
        ensure!(
            bought_asset_amount >= min_amount_out,
            Error::<T>::InsufficientOutputAmount,
        );

        let vault = Module::<T>::account_id();

        // Transfer sold token amount from user to vault and bought token amount from vault to user
        T::Currency::transfer(
            sold_asset_id.into(),
            &sender,
            &vault,
            sold_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;
        T::Currency::transfer(
            bought_asset_id.into(),
            &vault,
            &sender,
            bought_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;

        // Apply changes in token pools, adding sold amount and removing bought amount
        Pools::insert(
            (sold_asset_id, bought_asset_id),
            input_reserve + sold_asset_amount,
        );
        Pools::insert(
            (bought_asset_id, sold_asset_id),
            output_reserve - bought_asset_amount,
        );

        // Settle tokens which goes to treasury and for buy and burn purpose
        Module::<T>::settle_treasury_and_burn(sold_asset_id, bought_asset_id, sold_asset_amount)?;

        Module::<T>::deposit_event(RawEvent::AssetsSwapped(
            sender,
            sold_asset_id,
            sold_asset_amount,
            bought_asset_id,
            bought_asset_amount,
        ));

        Ok(())
    }

    fn buy_asset(
        sender: T::AccountId,
        sold_asset_id: Self::CurrencyId,
        bought_asset_id: Self::CurrencyId,
        bought_asset_amount: Self::Balance,
        max_amount_in: Self::Balance,
    ) -> DispatchResult {
        // Ensure pool exists
        ensure!(
            Pools::contains_key((sold_asset_id, bought_asset_id)),
            Error::<T>::NoSuchPool,
        );

        // Get token reserves
        let input_reserve = Pools::get((sold_asset_id, bought_asset_id));
        let output_reserve = Pools::get((bought_asset_id, sold_asset_id));

        // Ensure there are enough tokens in reserves
        ensure!(
            output_reserve > bought_asset_amount,
            Error::<T>::NotEnoughReserve,
        );

        // Ensure not buying zero amount
        ensure!(!bought_asset_amount.is_zero(), Error::<T>::ZeroAmount,);

        // Calculate amount to be paid from bought amount
        let sold_asset_amount =
            Module::<T>::calculate_buy_price(input_reserve, output_reserve, bought_asset_amount);

        // Ensure user has enought tokens to sell
        ensure!(
            T::Currency::free_balance(sold_asset_id.into(), &sender).into() >= sold_asset_amount,
            Error::<T>::NotEnoughAssets,
        );

        // Ensure paid amount is less then maximum allowed price
        ensure!(
            sold_asset_amount <= max_amount_in,
            Error::<T>::InsufficientInputAmount,
        );

        let vault = Module::<T>::account_id();

        // Transfer sold token amount from user to vault and bought token amount from vault to user
        T::Currency::transfer(
            sold_asset_id.into(),
            &sender,
            &vault,
            sold_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;
        T::Currency::transfer(
            bought_asset_id.into(),
            &vault,
            &sender,
            bought_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;

        // Apply changes in token pools, adding sold amount and removing bought amount
        Pools::insert(
            (sold_asset_id, bought_asset_id),
            input_reserve + sold_asset_amount,
        );
        Pools::insert(
            (bought_asset_id, sold_asset_id),
            output_reserve - bought_asset_amount,
        );

        // Settle tokens which goes to treasury and for buy and burn purpose
        Self::settle_treasury_and_burn(sold_asset_id, bought_asset_id, sold_asset_amount)?;

        Module::<T>::deposit_event(RawEvent::AssetsSwapped(
            sender,
            sold_asset_id,
            sold_asset_amount,
            bought_asset_id,
            bought_asset_amount,
        ));

        Ok(())
    }


    fn mint_liquidity(
        sender: T::AccountId,
        first_asset_id: Self::CurrencyId,
        second_asset_id: Self::CurrencyId,
        first_asset_amount: Self::Balance,
        expected_second_asset_amount: Option<Self::Balance>,
    ) -> DispatchResult {
        let vault = Module::<T>::account_id();

        // Ensure pool exists
        ensure!(
            (LiquidityAssets::contains_key((first_asset_id, second_asset_id))
                || LiquidityAssets::contains_key((second_asset_id, first_asset_id))),
            Error::<T>::NoSuchPool,
        );

        // Get liquidity token id
        let liquidity_asset_id = Module::<T>::get_liquidity_asset(first_asset_id, second_asset_id);

        // Ensure pool exists
        ensure!(
            (Pools::contains_key((first_asset_id, second_asset_id))
                || Pools::contains_key((second_asset_id, first_asset_id))),
            Error::<T>::NoSuchPool,
        );

        // Get token reserves
        let first_asset_reserve = Pools::get((first_asset_id, second_asset_id));
        let second_asset_reserve = Pools::get((second_asset_id, first_asset_id));
        let total_liquidity_assets: Self::Balance =
            T::Currency::total_issuance(liquidity_asset_id.into()).into();

        // Conversion to U256
        let first_asset_amount_u256: U256 = first_asset_amount.saturated_into::<u128>().into();
        let first_asset_reserve_u256: U256 = first_asset_reserve.saturated_into::<u128>().into();
        let second_asset_reserve_u256: U256 = second_asset_reserve.saturated_into::<u128>().into();
        let total_liquidity_assets_u256: U256 =
            total_liquidity_assets.saturated_into::<u128>().into();

        // Calculation of required second asset amount and received liquidity token amount
        let second_asset_amount_u256: U256 =
            first_asset_amount_u256 * second_asset_reserve_u256 / first_asset_reserve_u256 + 1;
        let liquidity_assets_minted_u256: U256 =
            first_asset_amount_u256 * total_liquidity_assets_u256 / first_asset_reserve_u256;

        // Conversion to Balance
        let second_asset_amount = second_asset_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Self::Balance>();
        let liquidity_assets_minted = liquidity_assets_minted_u256
            .saturated_into::<u128>()
            .saturated_into::<Self::Balance>();


        ensure!(
            second_asset_amount <= expected_second_asset_amount.unwrap_or(second_asset_amount),
            Error::<T>::SecondAssetAmountExceededExpectations,
        );
        
        // Ensure minting amounts are not zero
        ensure!(
            !first_asset_amount.is_zero() && !second_asset_amount.is_zero(),
            Error::<T>::ZeroAmount,
        );

        // Ensure user has enough first adn second token amount
        ensure!(
            T::Currency::free_balance(first_asset_id.into(), &sender).into() >= first_asset_amount,
            Error::<T>::NotEnoughAssets,
        );

        ensure!(
            T::Currency::free_balance(second_asset_id.into(), &sender).into()
                >= second_asset_amount,
            Error::<T>::NotEnoughAssets,
        );

        // Transfer of token amounts from user to vault
        T::Currency::transfer(
            first_asset_id.into(),
            &sender,
            &vault,
            first_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;
        T::Currency::transfer(
            second_asset_id.into(),
            &sender,
            &vault,
            second_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;

        // Creating new liquidity tokens to user
        T::Currency::mint(
            liquidity_asset_id.into(),
            &sender,
            liquidity_assets_minted.into(),
        )?;

        // Apply changes in token pools, adding minted amounts
        Pools::insert(
            (&first_asset_id, &second_asset_id),
            first_asset_reserve + first_asset_amount,
        );
        Pools::insert(
            (&second_asset_id, &first_asset_id),
            second_asset_reserve + second_asset_amount,
        );

        Module::<T>::deposit_event(RawEvent::LiquidityMinted(
            sender,
            first_asset_id,
            first_asset_amount,
            second_asset_id,
            second_asset_amount,
            liquidity_asset_id,
            second_asset_amount,
        ));

        Ok(())
    }

    fn burn_liquidity(
        sender: T::AccountId,
        first_asset_id: Self::CurrencyId,
        second_asset_id: Self::CurrencyId,
        liquidity_asset_amount: Self::Balance,
    ) -> DispatchResult {
        let vault = Module::<T>::account_id();

        // Ensure pool exists
        ensure!(
            Pools::contains_key((first_asset_id, second_asset_id)),
            Error::<T>::NoSuchPool,
        );

        // Get token reserves and liquidity asset id
        let first_asset_reserve = Pools::get((first_asset_id, second_asset_id));
        let second_asset_reserve = Pools::get((second_asset_id, first_asset_id));
        let liquidity_asset_id = Module::<T>::get_liquidity_asset(first_asset_id, second_asset_id);

        // Ensure user has enought liquidity tokens to burn
        ensure!(
            T::Currency::can_slash(
                liquidity_asset_id.into(),
                &sender,
                liquidity_asset_amount.into()
            ),
            Error::<T>::NotEnoughAssets,
        );
        let new_balance: Self::Balance =
            T::Currency::free_balance(liquidity_asset_id.into(), &sender).into()
                - liquidity_asset_amount;

        T::Currency::ensure_can_withdraw(
            liquidity_asset_id.into(),
            &sender,
            liquidity_asset_amount.into(),
            WithdrawReasons::all(),
            new_balance.into(),
        )
        .or(Err(Error::<T>::NotEnoughAssets))?;

        // Calculate first and second token amounts depending on liquidity amount to burn
        let (first_asset_amount, second_asset_amount) =
            Module::<T>::get_burn_amount(first_asset_id, second_asset_id, liquidity_asset_amount);

        // Ensure not withdrawing zero amounts
        ensure!(
            !first_asset_amount.is_zero() && !second_asset_amount.is_zero(),
            Error::<T>::ZeroAmount,
        );

        // Transfer withdrawn amounts from vault to user
        T::Currency::transfer(
            first_asset_id.into(),
            &vault,
            &sender,
            first_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;
        T::Currency::transfer(
            second_asset_id.into(),
            &vault,
            &sender,
            second_asset_amount.into(),
            ExistenceRequirement::KeepAlive,
        )?;

        // Apply changes in token pools, removing withdrawn amounts
        Pools::insert(
            (&first_asset_id, &second_asset_id),
            first_asset_reserve - first_asset_amount,
        );
        Pools::insert(
            (&second_asset_id, &first_asset_id),
            second_asset_reserve - second_asset_amount,
        );

        // Destroying token pool, if no tokens left
        if (first_asset_reserve - first_asset_amount == 0.saturated_into::<Self::Balance>())
            || (second_asset_reserve - second_asset_amount == 0.saturated_into::<Self::Balance>())
        {
            Pools::remove((first_asset_id, second_asset_id));
            Pools::remove((second_asset_id, first_asset_id));
        }

        // Destroying burnt liquidity tokens
        T::Currency::burn_and_settle(
            liquidity_asset_id.into(),
            &sender,
            liquidity_asset_amount.into(),
        )?;

        Module::<T>::deposit_event(RawEvent::LiquidityBurned(
            sender,
            first_asset_id,
            first_asset_amount,
            second_asset_id,
            second_asset_amount,
            liquidity_asset_id,
            second_asset_amount,
        ));

        Ok(())
    }

    // This function has not been verified
    fn get_tokens_required_for_minting(
        liquidity_asset_id: Self::CurrencyId,
        liquidity_token_amount: Self::Balance,
    ) -> (
        Self::CurrencyId,
        Self::Balance,
        Self::CurrencyId,
        Self::Balance,
    ) {
        let (first_asset_id, second_asset_id) = LiquidityPools::get(liquidity_asset_id);
        let first_asset_reserve = Pools::get((first_asset_id, second_asset_id));
        let second_asset_reserve = Pools::get((second_asset_id, first_asset_id));
        let total_liquidity_assets: Balance =
            T::Currency::total_issuance(liquidity_asset_id.into()).into();

        let liquidity_token_amount_u256: U256 =
            liquidity_token_amount.saturated_into::<u128>().into();
        let first_asset_reserve_u256: U256 = first_asset_reserve.saturated_into::<u128>().into();
        let second_asset_reserve_u256: U256 = second_asset_reserve.saturated_into::<u128>().into();
        let total_liquidity_assets_u256: U256 =
            total_liquidity_assets.saturated_into::<u128>().into();

        let second_asset_amount_u256: U256 =
            liquidity_token_amount_u256 * second_asset_reserve_u256 / total_liquidity_assets_u256
                + 1;
        let first_asset_amount_u256: U256 = liquidity_token_amount_u256 * first_asset_reserve_u256
            / total_liquidity_assets_u256
            + 1;

        let second_asset_amount = second_asset_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Balance>();
        let first_asset_amount = first_asset_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Balance>();

        (
            first_asset_id,
            first_asset_amount,
            second_asset_id,
            second_asset_amount,
        )
    }
}

pub trait Valuate {
    type Balance: AtLeast32BitUnsigned
        + FullCodec
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Default
        + From<Balance>
        + Into<Balance>;

    type CurrencyId: Parameter
        + Member
        + Copy
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + AtLeast32BitUnsigned
        + FullCodec
        + From<TokenId>
        + Into<TokenId>;

    fn get_liquidity_token_mng_pool(
        liquidity_token_id: Self::CurrencyId,
    ) -> Result<(Self::CurrencyId, Self::CurrencyId), DispatchError>;

    fn valuate_liquidity_token(
        liquidity_token_id: Self::CurrencyId,
        liquidity_token_amount: Self::Balance,
    ) -> Self::Balance;

    fn scale_liquidity_by_mng_valuation(
        mng_valuation: Self::Balance,
        liquidity_token_amount: Self::Balance,
        mng_token_amount: Self::Balance,
    ) -> Self::Balance;
}

impl<T: Trait> Valuate for Module<T> {
    type Balance = Balance;

    type CurrencyId = TokenId;

    fn get_liquidity_token_mng_pool(
        liquidity_token_id: Self::CurrencyId,
    ) -> Result<(Self::CurrencyId, Self::CurrencyId), DispatchError> {
        let (first_token_id, second_token_id) = LiquidityPools::get(liquidity_token_id);
        let native_currency_id = T::NativeCurrencyId::get();
        match native_currency_id {
            _ if native_currency_id == first_token_id => Ok((first_token_id, second_token_id)),
            _ if native_currency_id == second_token_id => Ok((second_token_id, first_token_id)),
            _ => Err(Error::<T>::NotMangataLiquidityAsset.into()),
        }
    }

    fn valuate_liquidity_token(
        liquidity_token_id: Self::CurrencyId,
        liquidity_token_amount: Self::Balance,
    ) -> Self::Balance {
        let (mng_token_id, other_token_id) =
            match Self::get_liquidity_token_mng_pool(liquidity_token_id) {
                Ok(pool) => pool,
                Err(_) => return Default::default(),
            };
        let mng_token_reserve = Pools::get((mng_token_id, other_token_id));
        let liquidity_token_reserve = T::Currency::total_issuance(liquidity_token_id.into());
        let mng_token_reserve_u256: U256 = mng_token_reserve.saturated_into::<u128>().into();
        let liquidity_token_amount_u256: U256 =
            liquidity_token_amount.saturated_into::<u128>().into();
        let liquidity_token_reserve_u256: U256 =
            liquidity_token_reserve.saturated_into::<u128>().into();
        let mng_token_amount_u256 =
            mng_token_reserve_u256 * liquidity_token_amount_u256 / liquidity_token_reserve_u256;
        let mng_token_amount = mng_token_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Self::Balance>();
        mng_token_amount
    }

    fn scale_liquidity_by_mng_valuation(
        mng_valuation: Self::Balance,
        liquidity_token_amount: Self::Balance,
        mng_token_amount: Self::Balance,
    ) -> Self::Balance {
        if mng_valuation.is_zero() {
            return 0u128.into();
        }
        let mng_valuation_u256: U256 = mng_valuation.saturated_into::<u128>().into();
        let liquidity_token_amount_u256: U256 =
            liquidity_token_amount.saturated_into::<u128>().into();
        let mng_token_amount_u256: U256 = mng_token_amount.saturated_into::<u128>().into();
        let scaled_liquidity_token_amount_u256 =
            liquidity_token_amount_u256 * mng_token_amount_u256 / mng_valuation_u256;
        let scaled_liquidity_token_amount = scaled_liquidity_token_amount_u256
            .saturated_into::<u128>()
            .saturated_into::<Self::Balance>();
        scaled_liquidity_token_amount
    }
}
