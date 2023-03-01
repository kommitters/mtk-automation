# MTK Automation

[![License badge](https://img.shields.io/hexpm/l/kadena?style=for-the-badge)](https://github.com/kommitters/kadena.ex/blob/main/LICENSE)

A smart contract written in Soroban to reward members of an organization who perform meaningful tasks.

## Contract workflow
1. Generate an admin account.
2. Create and initialize the stellar token. This step relies on the built-in token contract.
    > **Note** The initialization for the token contract must be skipped if the token already exists in the stellar network.
3. Initialize the organization contract with your custom rewards.
4. Generate a signature to fund the organization's token balance using the administrator's account.
5. Fund the balance of the contract using the previously generated signature.
6. Add members to the organization.
7. Create a signature to enable token transfer for the accounts.
    > **Note** A signature is required for each transfer transaction.
8. Reward members.

> **Note** Signatures are required to execute any functions involving calls to **privileged functions** of the token contract. [Token Contract Interface](https://soroban.stellar.org/docs/common-interfaces/token).

## Revoke membership
1. Approve the transaction using the token contract.
2. Transfer the balance to the organization by revoking the membership.

## Setup
For setting up your environment, visit: [Soroban setup](https://soroban.stellar.org/docs/getting-started/setup)

## Testing
For testing the contract run `cargo test -- --show-output`

## Deploying and using our contracts 
All this steps require the [Setup](#setup) 

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

3. Initialize the `token_exchange` contract with your exchange tokens and prices.

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

4. Initialize the organization contract with your custom rewards.

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

5. Add a member 
    > Create the member in the same way as you did the admin in the stellar laboratory.
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

6. Offset a member 
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

7. Revoke a member (this was done in three stages because the `CLI` isn't currently working with multiple signature functions).
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

#### **token_exchange**

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

4. To retrieve the price assigned to the exchange of tokens 

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

#### **mtk_automation**

1. To retrieve the members that are in the organization 
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

3. To retrieve the token id that is used to assign offsets
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
