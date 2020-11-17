use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};
use sp_std::vec::Vec;

/// A type implementing this trait stores all information of an NFT.
pub trait Nft {
    /// The type used to identify unique assets.
    type Id;
    /// The attributes that distinguish unique assets.
    type Info;
}

/// A type implementing Unique is thought of as unique in the non-fungible sense.
/// It will be
/// - ownable
/// - transferable
/// - possible to reason about the cardinality of sets of this type
pub trait Unique {
    /// A struct that implements the Nft trait.
    type Asset: Nft;
    /// The type used to identify asset owners.
    type AccountId;

    /// The ID of the account that owns an asset.
    fn owner_of(asset_id: &<Self::Asset as Nft>::Id) -> Option<Self::AccountId>;

    /// Transfer ownership of an asset to another account.
    /// This method **must** return an error in the following cases:
    /// - The asset with the specified ID does not exist.
    /// - The destination account has already reached the user asset limit.
    fn transfer(caller: &Self::AccountId,
                dest_account: &Self::AccountId,
                asset_id: &<Self::Asset as Nft>::Id) -> DispatchResult;
}

/// A Mintable type is capable of being instantiated.
pub trait Mintable {
    /// A struct that implements the Nft trait.
    type Asset: Nft;
    /// The type used to identify asset owners.
    type AccountId;

    /// Use the provided asset info to create a new unique asset for the specified user.
    /// This method **must** return an error in the following cases:
    /// - The asset, as identified by the asset info, already exists.
    /// - The specified owner account has already reached the user asset limit.
    /// - The total asset limit has already been reached.
    fn mint(caller: &Self::AccountId,
            owner: &Self::AccountId,
            asset_id: &<Self::Asset as Nft>::Id,
            asset_info: <Self::Asset as Nft>::Info,
    ) -> Result<(), DispatchError>;
}

/// An instance of a Burnable type can be destroyed.
pub trait Burnable {
    /// A struct that implements the Nft trait.
    type Asset: Nft;
    /// The type used to identify asset owners.
    type AccountId;

    /// Destroy an asset.
    /// This method **must** return an error in the following case:
    /// - The asset with the specified ID does not exist.
    fn burn(caller: Self::AccountId, asset_id: &<Self::Asset as Nft>::Id) -> DispatchResult;
    /// The total number of this type of asset that has been burned (may overflow).
    fn burned() -> u128;
}

/// An instance of a Cappable type has a limit on the maximum amount of assets
/// allowed per user and in total.
pub trait Cappable {
    /// A struct that implements the Nft trait.
    type Asset: Nft;
    type AccountId;

    /// The total number of this type of asset that exists (minted - burned).
    fn total() -> u128;
    /// The total number of this type of asset owned by an account.
    fn total_for_account(account: &Self::AccountId) -> u64;
    /// Total number of assets allowed to exist. An implementor *MUST* check that
    /// the total does not exceed the limit.
    type AssetLimit: Get<u128>;
    /// Maximum number of assets a single user is allowed to own. An implementor
    /// *MUST* check that the total does not exceed the limit.
    type UserAssetLimit: Get<u64>;
}
