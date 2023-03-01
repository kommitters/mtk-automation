# MTK Automation

[![License badge](https://img.shields.io/hexpm/l/kadena?style=for-the-badge)](https://github.com/kommitters/kadena.ex/blob/main/LICENSE)

MTK Automation is a repository consisting of two smart contracts built using `Soroban SDK`, designed to facilitate member management and offset them for specific organizational tasks. The contracts are as follows:

- **token_exchange**: This contract manages the exchange price of two tokens, enabling an organization to exchange its offset tokens for other token defined by it. e.g: The organization is offering that 1 USDT can be claimed for every 1000 CustomORGTK.
- **mtk-automation**: This contract allows organizations to add new members, search for existing members, and revoke membership. When revoking members, this contract automatically invokes the `token_exchange` contract, which exchanges the offset token balance of the member being removed from the organization with the one that is already defined in the contract.

By utilizing MTK Automation, organizations can efficiently manage their members and offset tokens, streamlining their administrative processes.

## Contracts workflow
- Generate an admin account.
- Create and initialize the stellar token. This step relies on the built-in token contract.
    > **Note** The initialization for the token contract must be skipped if the token already exists in the stellar network.

### Exchange Contract workflow
1. Deploy and initialize the `exchange_token` contract with your prices and the `sell`/`buy` token contracts ids.
2. Mint the contract with the quantity of the `sell` token you want to be available to exchange.

### Organization Contract workflow
1. Deploy and initialize the organization contract with your custom offsets.
2. Add members to the organization.
3. Offset members.
4. Revoke membership
    - For this, the member is the one that approves the transaction being the `invoker` (setting his secret key when calling the revoke function).
    - Trade the member balance an return the tokens to the organization by "swapping" the token with any other (ideally a stable one).
    - Revoke the member from the organization.
> **Note** The Secret key of the account is the one that verifies if the transactions are allowed to be executed by the `invoker` of the contract.

## Pre-requirements
To be able to work with the contracts, you first need some programs or dependencies, for this you can read the following guide: [Soroban setup](https://soroban.stellar.org/docs/getting-started/setup).

## Setup
``` 
# Clone the repository 
git clone git@github.com:kommitters/mtk-automation.git

# Build the project and get dependencies 
cd mtk-automation
cargo build
 ```

## Testing
To test the contract run `cargo test -- --show-output` or ` cargo test -- --nocapture`

## Contracts deployment and usage
All this steps require the [Pre-requirements](#pre-requirements) and [Setup](#setup) 

1. Generate an admin account in the [Futurenet](https://laboratory.stellar.org/#account-creator?network=futurenet)
2. Create and initialize two tokens contracts.
    >  **Note** You can do it downloading the [token example](https://github.com/stellar/soroban-examples/tree/main/token), deploy and initialize them, also remember to mint the admin account with both tokens.
3. Build both contracts with `cargo build --target wasm32-unknown-unknown --release`
4. Deploy `token_exchange` contract 
```
soroban contract deploy \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm

sucess
sucess
<token_exchange_id>
```
5. Deploy `mtk-automation` contract 
```CLI
soroban contract deploy \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm

sucess
sucess
<mtk_automation_id>
```

5. Initialize the `token_exchange` contract with your exchange tokens(those you created in step `#2`) and exchange prices.

```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <token_exchange_id> \
    --fn create \
    -- \
    --seller <admin-public-key> \
    --sell_token <sell-token-contract-id> \
    --buy_token <buy-token-contract-id> \
    --sell_price 1 \
    --buy_price 1
```
Mint the contract with some `sell_token` so the contract can be the "middleman" in the process
``` 
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <token_exchange_id> \
    --fn mint_cont \
    --\
    --token <sell-token-contract-id> \
    --amount 1000
```

6. Initialize the organization contract with your custom offsets.
    > Note that the `<buy-token-contract-id>` is the token that we want to use to offset the members 

``` 
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn initialize \
    -- \
    --admin <admin-public-key> \
    --org_name 'Kommit' \
    --offsets '{"thank": {"i32": 30} }' \
    --fund_amount 10000 \
    --token_c_id '<buy-token-contract-id>' \
    --exchange_c_id '<token_exchange_id>'

```

7. Add a member 
    > You can create the member in the same way as you did the admin in the stellar laboratory.
```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn add_m \
    -- \
    --account <member-public-key> \
    --admin <admin-public-key>

```

8. Offset a member 
``` 
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id 81a660aa54aa251ac761feadd00d35bfcfd529cb679bade331ed9203d2c42832 \
    --fn offset_m \
    -- \
    --admin_address <admin-public-key> \
    --to <member-public-key> \
    --o_type "thank"

```

9. Revoke a member (this was done in three stages because the `CLI` isn't currently working with multiple signature functions).
```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <member-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn revoke_m1 \
    -- \
    --from <member-public-key>

soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <member-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn revoke_m2 \
    -- \
    --from <member-public-key>

soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <member-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn revoke_m3 \
    -- \
    --from <member-public-key>

```

### Additional commands

#### **token_exchange contract**

1. To retrieve the balance of any token in the contract
```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <token_exchange_id> \
    --fn get_c_bal \
    -- \
    --token <any-token-id>
```
2. To update the price of the tokens that are being sold and bought

``` 
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <token_exchange_id> \
    --fn updt_price \
    -- \
    --sell_price <new-sell-price>
    --buy_price <new-buy-price>

```

3. To withdraw the balance in the contract of any token 

```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <token_exchange_id> \
    --fn withdraw \
    -- \
    --token <any-token-id>
    --amount 100
```

4. To retrieve the offer assigned to exchange tokens 

```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/token_exchange.wasm \
    --secret-key <admin-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <token_exchange_id> \
    --fn withdraw \
    -- \
    --get_offer
```

#### **mtk_automation contract**

1. To retrieve the members of the organization 
```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <any-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn get_m

```

2. To retrieve the organization name
```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <any-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn org_name
```

3. To retrieve the id of the token used to assign offsets
```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/mtk_automation.wasm \
    --secret-key <any-secret-key> \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <mtk_automation_id> \
    --fn get_tc_id

```
## Changelog

Features and bug fixes are listed in the [CHANGELOG][changelog] file.

## Code of conduct

We welcome everyone to contribute. Make sure you have read the [CODE_OF_CONDUCT][coc] before.

## Contributing

For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

## License

This library is licensed under an MIT license. See [LICENSE][license] for details.

## Acknowledgements

Made with 💙 by [kommitters Open Source](https://kommit.co)

[license]: https://github.com/kommitters/.template/blob/main/LICENSE
[coc]: https://github.com/kommitters/.template/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/.template/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/.template/blob/main/CONTRIBUTING.md
