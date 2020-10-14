## Unique Assets
#### Modular trait definitions for Non-Fungible Tokens

---
This library defines a set of traits which can be used in defining properties
of a non-fungible token (nft) in Rust. The definitions strive to be universal
and are compatible with the ERC721 standard on Ethereum.

### Implementing for a Substrate Pallet
Note that the traits do not enforce anything about the user-facing dispatchable
methods. Instead, traits are implemented by internal methods.

```
impl<T: Trait> Unique for Module<T> {
    type Asset = ..;
    type AccountId = ..;

    fn owner_of(asset_id: &<Self::Asset as Nft>::Id) -> DispatchResult {
        ...
    }
    fn transfer(caller: &Self::AccountId,
                dest_account: &Self::AccountId,
                asset_id: &<Self::Asset as Nft>::Id) -> DispatchResult {
        ...
    }

}
```

Internal methods determine the composability of a pallet.
For a pallet to be maximally reusable, it should implement a common interface,
and only expose dispatchable methods when it really makes sense.
