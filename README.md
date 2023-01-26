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

[license]: https://github.com/kommitters/mtk-automation/blob/main/LICENSE
[coc]: https://github.com/kommitters/mtk-automation/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/mtk-automation/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/mtk-automation/blob/main/CONTRIBUTING.md
