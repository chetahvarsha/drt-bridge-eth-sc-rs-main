# Dharitri-Ethereum Bridge

## Abstract
In this repository, you will find smart contracts on the Dharitri side for the Dharitri-Ethereum bridge. For setup details, take a look [here](docs/setup.md). Documentation for relayers can be found [here](docs/relayer.md).  

Although cryptocurrency is the future, there are still many challenges to overcome. One of them is providing an easy way to manage multiple different types of coins. Although we already have some exchange services, they can be a bit difficult to use and require many steps to perform the actual transfer, and they require you to trust a centralized third party and/or a centralized ERC20 token smart contract. With this suite of smart contracts, we aim to provide a decentralized way of transferring tokens between Dharitri and Ethereum.  

For this to be truly decentralized, we're using a multi-sig smart contract. More info on that [here](https://github.com/dharitri/drt-sdk-rs/blob/master/contracts/examples/multisig/README.md).  We're going to have a set of relayers that validate the transaction. You could think of it as a mini-blockchain, whose sole purpose is to handle cross-chain transactions.  

To be able to transfer the tokens, we make use of a concept known as "Wrapped Tokens". You don't transfer native REWA or ETH tokens, but instead, you lock the native tokens in a contract and generate "Wrapped" versions of them. On Dharitri, we are going to use DCDT for this purpose. On Ethereum, we're most likely going to use something like ERC20.  

Now you might ask, how is this any different? There already is an ERC20-style contract on Ethereum for REWA! The main difference is you won't have to go through exchange services at all, and at some point, we'll likely have this integrated fully into our xPortal application. And there's one more important difference: This is decentralized. There isn't a single contract owner doing all the work.  We have a set of trusted accounts that will handle the transactions.  

But why should _you_ trust them? We use the "Proof of Stake" concept: each of them will have to stake a certain amount of REWA to be able to become a relayer, just like validators on the Dharitri blockchain. If any of them misbehaves, their stake will be "slashed" and they'll lose quite a bit of money as a result.

## Wrapped Tokens

As said above, we're going to use DCDT to implement wrapped tokens, but how is the wrapping done? For that, we have the `RewaDcdtSwap`, a very simple SC, whose only purpose is to exchange 1:1 native REWA to WrappedRewa DCDT tokens. You can also do the reverse operation at any time, which is known as unwrapping.  

One important thing to note is you can never unwrap your WrappedETH while on the Dharitri blockchain, as that is not a native Dharitri token. You will only be able to unwrap them by transferring them to one of your Ethereum accounts and then unwrapping them there.  


## Dharitri -> Ethereum transaction

To be able to send REWA to an Ethereum account, we first have to wrap the tokens through the `RewaDcdtSwap` contract.   

Then you can create a transaction by making a smart contract call to the `DcdtSafe` SC with the tokens you want to transfer and the receiver's address. The tokens will be locked in the contract until the transaction is processed. If the transaction is successful, the tokens on the Dharitri side will be burned. If the transaction fails for whatever reason, you will get your tokens back.  

Note that not all tokens will be transferred, part of them will be deducted for transaction fees.  

## Ethereum -> Dharitri transaction

To be able to transfer your tokens back, you will likely have to use an ERC20 contract on the Ethereum blockchain. Once your transaction has been processed on that side, our relayers will simply transfer the tokens back to your Dharitri account, through the `MultiTransferDcdt` SC. No additional fees have to be paid for this kind of transaction.  

## Conclusion

And that sums up the Dharitri-Ethereum bridge. It's open source, so if you're interested in the details, you can always check out the implementation. In the future, it will likely be implemented in xPortal, so it will be very straightforward to move your tokens around :)

