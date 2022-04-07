# Giveaway Smart Contract
## Purpose
This smart contract exists to choose the Tangle Dragons' giveaway winners transparently. All participant's metamask addresses are uploaded to the smart contract, and then a number of winners is randomly selected.
## Functions
The Giveaway Smart contract has 3 main functions:
* loadAddresses()
* unloadAddresses()
* ruffle()
**loadAddresses()** loads the participant's metamask addresses to the smart contract. If there are too many participants, the input size will exceed the limit and many calls to this function will have to be made. Note that addresses must not include the "0x" at the beginning.
**unloadAddresses()** resets the list of the participants for a new giveaway to take place afterwards.
**ruffle()** selects n winners out of the participants and makes an event with the results. It takes one argument that is “n” (the number of winners).

## Check results
If we want to be transparent, the idea is to be able to check who participated on the giveaway and who has the smart contract chosen as the winners.
### Check results Giveaway #1
The Giveaway Smart Contract is currently running on the zentangle chain (chain ID: gVWjq4h6jCX5ZKgeYTnC8LbpouGRT4sbpEWNuMoKe6CS). This means, you can access all interactions that have happened with the Giveaway Smart Contract on our [Dashboard](https://dash-wasp.zentangle.io/chain/gVWjq4h6jCX5ZKgeYTnC8LbpouGRT4sbpEWNuMoKe6CS/contract/31774d34). Use:

username: wasp

password: wasp
### Check results Giveaway #2
As the first Giveaway SC has crashed doe to network stability issues of Goshimmer, a new Smart Contract was published. This time it's currently running on the Giveaway Chain (chain ID: gLruuKrc7BNUapPi95MMCLguaAqKjM6sQPjw95yo5iSV). This means, you can access all interactions that have happened with the Giveaway Smart Contract on our [Dashboard](http://test-dcsc.zentangle.io:7000/chain/gLruuKrc7BNUapPi95MMCLguaAqKjM6sQPjw95yo5iSV/block/7). Use:

username: wasp

password: wasp
![image](https://user-images.githubusercontent.com/41125296/162103300-723db9bc-48d5-41c4-92b2-730ad07d88ce.png)
### Check events trough CLI
Sometimes, if the inputs or outputs are too big, they get cut and not all the information can be seen.
In order to see it, you can do so through the wasp-cli.
In the [iota wiki](https://wiki.iota.org/smart-contracts/guide/chains_and_nodes/running-a-node) there is a tutorial on how to set it up. There is no need to set up a node, since you will need to connect to ours. Just put on your CLI:
`wasp-cli set https://api.goshimmer.sc.iota-defi.com`
`wasp-cli set wasp.0.api https://test-dcsc.zentangle.io/`
and you will be able to connect.


Then, write 
`wasp-cli chain request <ID>`


You can find the ID in the same dashboard when looking at the block where the event in question happened.

Inputs and outputs are in hexadecimal. Just convert them to string on https://codebeautify.org/hex-string-converter

## Give Away Winners
Tangle Dragon Genesis NFT ID - Metamask Address of Winner
1. d959df95169bff5d293df05817c3a5d19047177e - 1144
2. 9b9c5915dabefbd92db77782d4bf54f0356f515b - 2052
3. d8206a0ac8d275a88842d7064e9e682575a11e3f - 2436
4. 7637e334de9ff6dfdd0fbac5bd13ac7ca3cffdb7 - 1382
5. e954bbabbe298215baa94d9a89fc100529fc2652 - 2100
6. d17be40bb9256b90325272a52ad22f661fd9cb19 - 662
7. 5a22618a380a56c20a0c26664c3955d594f426e8 - 96
