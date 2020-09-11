use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};

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
// TODO: Is this reasonable bcs it will be placed as a macro in a file with a Trait declaration?
//pub trait Unique<T: Trait> {
    /// A struct that implements the Nft trait.
    type Asset: Nft;
    /// The type used to identify asset owners.
    type AccountId;

    // TODO: Should not be part of unique assets, make a Cappable trait
    type AssetLimit: Get<u128>;
    type UserAssetLimit: Get<u64>;


    /// The total number of this type of asset that exists (minted - burned).
    fn total() -> u128;
    /// The total number of this type of asset owned by an account.
    fn total_for_account(account: &Self::AccountId) -> u64;
    /// The set of unique assets owned by an account.
    fn assets_for_account(account: &Self::AccountId) -> Vec<Self::Asset>;
    /// The ID of the account that owns an asset.
    fn owner_of(asset_id: &<Self::Asset as Nft>::Id) -> Self::AccountId;

    /// Transfer ownership of an asset to another account.
    /// This method **must** return an error in the following cases:
    /// - The asset with the specified ID does not exist.
    /// - The destination account has already reached the user asset limit.
    fn transfer(dest_account: &Self::AccountId, asset_id: &<Self::Asset as Nft>::Id) -> DispatchResult;
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
    fn mint(owner: &Self::AccountId,
            asset_info: <Self::Asset as Nft>::Info,
    ) -> Result<<Self::Asset as Nft>::Id, DispatchError>;
}

/// An instance of a Burnable type can be destroyed.
pub trait Burnable {
    /// A struct that implements the Nft trait.
    type Asset: Nft;

    /// Destroy an asset.
    /// This method **must** return an error in the following case:
    /// - The asset with the specified ID does not exist.
    fn burn(asset_id: &<Self::Asset as Nft>::Id) -> DispatchResult;
    /// The total number of this type of asset that has been burned (may overflow).
    fn burned() -> u128;
}