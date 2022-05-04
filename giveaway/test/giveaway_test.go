// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/iotaledger/wasp/giveaway/go/giveaway"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, giveaway.ScName, giveaway.OnLoad)
	require.NoError(t, ctx.ContractExists(giveaway.ScName))
}

func Test(t *testing.T) {
	*wasmsolo.RsWasm = true
	ctx := wasmsolo.NewSoloContext(t, giveaway.ScName, giveaway.OnLoad)

	number_of_winners := int16(10)
	const addresses = `{
		"addresses": [
			"622c1aaa66fb277a80a6f250",
			"622c1aaacdb70a4f242f7144",
			"622c1aaac9f92f8edba75423",
			"622c1aaab1af08ceb7d9cf9e",
			"622c1aaa8cb70df50de0e35e",
			"622c1aaa4cafc593d9605f7b",
			"622c1aaaba5731e2dade64ae",
			"622c1aaad7af934629bd93f0",
			"622c1aaa48c33fa1b25782d8",
			"622c1aaac6ae8d809fd57e4d",
			"622c1aaa2f3fe703526437cf",
			"622c1aaaf0919b2b332e84a0",
			"622c1aaa094b720878459be3",
			"622c1aaae45099da8e322805",
			"622c1aaaa9d0256ab9bae936",
			"622c1aaac2aeaeb381b443fd",
			"622c1aaa79f61c9f77b63c06",
			"622c1aaab17b7ca3e4643f5f",
			"622c1aaadeca6ac12bef6d93",
			"622c1aaa6f38489ff7bbc8d5",
			"622c1aaa77db244518736468",
			"622c1aaae62e62cfb0452f5c",
			"622c1aaae6b40a421e35e43b",
			"622c1aaadfc54e4c351896ec",
			"622c1aaa0a518410c7520330",
			"622c1aaa30f98a9a91911d54",
			"622c1aaa3981c83e51391063",
			"622c1aaa93a1dbe492460bfe",
			"622c1aaacc8c890e5b03df45",
			"622c1aaae6af12bd41b1a870",
			"622c1aaa89e2a32b28b8b219",
			"622c1aaa0695ca6c41eff878",
			"622c1aaa9747568ff00db296",
			"622c1aaaf05888d9dc4f4ff4",
			"622c1aaa04f47698d58e5012",
			"622c1aaa7b57a5a04e3a5dd5",
			"622c1aaa000c76e09d2deea1",
			"622c1aaa96813890883437c8",
			"622c1aaae6f65c79fd98ded6",
			"622c1aaa7cc4ea92b8a65332",
			"622c1aaa7daa32e87cb0d067",
			"622c1aaad4f18be1e20de2a6",
			"622c1aaa413abdc400172499",
			"622c1aaa7dd5fc716625d9e1",
			"622c1aaae82bc16b9890a831",
			"622c1aaae6cd3e61b22e0415",
			"622c1aaa7c0d45e0610324cc",
			"622c1aaa832cd99cf62b8830",
			"622c1aaa715b1c3b902250e6",
			"622c1aaa750d4fe375a244a2",
			"622c1aaa66fb277a80a6f250",
			"622c1aaacdb70a4f242f7144",
			"622c1aaac9f92f8edba75423",
			"622c1aaab1af08ceb7d9cf9e",
			"622c1aaa8cb70df50de0e35e",
			"622c1aaa4cafc593d9605f7b",
			"622c1aaaba5731e2dade64ae",
			"622c1aaad7af934629bd93f0",
			"622c1aaa48c33fa1b25782d8",
			"622c1aaac6ae8d809fd57e4d",
			"622c1aaa2f3fe703526437cf",
			"622c1aaaf0919b2b332e84a0",
			"622c1aaa094b720878459be3",
			"622c1aaae45099da8e322805",
			"622c1aaaa9d0256ab9bae936",
			"622c1aaac2aeaeb381b443fd",
			"622c1aaa79f61c9f77b63c06",
			"622c1aaab17b7ca3e4643f5f",
			"622c1aaadeca6ac12bef6d93",
			"622c1aaa6f38489ff7bbc8d5",
			"622c1aaa77db244518736468",
			"622c1aaae62e62cfb0452f5c",
			"622c1aaae6b40a421e35e43b",
			"622c1aaadfc54e4c351896ec",
			"622c1aaa0a518410c7520330",
			"622c1aaa30f98a9a91911d54",
			"622c1aaa3981c83e51391063",
			"622c1aaa93a1dbe492460bfe",
			"622c1aaacc8c890e5b03df45",
			"622c1aaae6af12bd41b1a870",
			"622c1aaa89e2a32b28b8b219",
			"622c1aaa0695ca6c41eff878",
			"622c1aaa9747568ff00db296",
			"622c1aaaf05888d9dc4f4ff4",
			"622c1aaa04f47698d58e5012",
			"622c1aaa7b57a5a04e3a5dd5",
			"622c1aaa000c76e09d2deea1",
			"622c1aaa96813890883437c8",
			"622c1aaae6f65c79fd98ded6",
			"622c1aaa7cc4ea92b8a65332",
			"622c1aaa7daa32e87cb0d067",
			"622c1aaad4f18be1e20de2a6",
			"622c1aaa413abdc400172499",
			"622c1aaa7dd5fc716625d9e1",
			"622c1aaae82bc16b9890a831",
			"622c1aaae6cd3e61b22e0415",
			"622c1aaa7c0d45e0610324cc",
			"622c1aaa832cd99cf62b8830",
			"622c1aaa715b1c3b902250e6",
			"622c1aaa750d4fe375a244a2",
			622c1aaa66fb277a80a6f250",
			"622c1aaacdb70a4f242f7144",
			"622c1aaac9f92f8edba75423",
			"622c1aaab1af08ceb7d9cf9e",
			"622c1aaa8cb70df50de0e35e",
			"622c1aaa4cafc593d9605f7b",
			"622c1aaaba5731e2dade64ae",
			"622c1aaad7af934629bd93f0",
			"622c1aaa48c33fa1b25782d8",
			"622c1aaac6ae8d809fd57e4d",
			"622c1aaa2f3fe703526437cf",
			"622c1aaaf0919b2b332e84a0",
			"622c1aaa094b720878459be3",
			"622c1aaae45099da8e322805",
			"622c1aaaa9d0256ab9bae936",
			"622c1aaac2aeaeb381b443fd",
			"622c1aaa79f61c9f77b63c06",
			"622c1aaab17b7ca3e4643f5f",
			"622c1aaadeca6ac12bef6d93",
			"622c1aaa6f38489ff7bbc8d5",
			"622c1aaa77db244518736468",
			"622c1aaae62e62cfb0452f5c",
			"622c1aaae6b40a421e35e43b",
			"622c1aaadfc54e4c351896ec",
			"622c1aaa0a518410c7520330",
			"622c1aaa30f98a9a91911d54",
			"622c1aaa3981c83e51391063",
			"622c1aaa93a1dbe492460bfe",
			"622c1aaacc8c890e5b03df45",
			"622c1aaae6af12bd41b1a870",
			"622c1aaa89e2a32b28b8b219",
			"622c1aaa0695ca6c41eff878",
			"622c1aaa9747568ff00db296",
			"622c1aaaf05888d9dc4f4ff4",
			"622c1aaa04f47698d58e5012",
			"622c1aaa7b57a5a04e3a5dd5",
			"622c1aaa000c76e09d2deea1",
			"622c1aaa96813890883437c8",
			"622c1aaae6f65c79fd98ded6",
			"622c1aaa7cc4ea92b8a65332",
			"622c1aaa7daa32e87cb0d067",
			"622c1aaad4f18be1e20de2a6",
			"622c1aaa413abdc400172499",
			"622c1aaa7dd5fc716625d9e1",
			"622c1aaae82bc16b9890a831",
			"622c1aaae6cd3e61b22e0415",
			"622c1aaa7c0d45e0610324cc",
			"622c1aaa832cd99cf62b8830",
			"622c1aaa715b1c3b902250e6",
			"622c1aaa750d4fe375a244a2"
		]
	  }`
	const addresses1 = `{
		"addresses": [
			"622ce49393ac82ceac5ed024",
			"622ce4937e093a0c75962fa9",
			"622ce49356dbb0eea663200d",
			"622ce493ef6cad480a3afdf1",
			"622ce493296c8d66c12b90b2",
			"622ce4932c13a92e6b6752d1",
			"622ce4938764e09252e6de63",
			"622ce49315fa7cc0d2ac61ee",
			"622ce4935e0920c0e02b035d",
			"622ce49339f85782bc3ee1b2",
			"622ce493b4e309123236b2e3",
			"622ce493d112a8589386d46a",
			"622ce493d1d52530e6c2a556",
			"622ce49323e9ec79d303b522",
			"622ce493310f4f7a0630f240",
			"622ce4930724a1f97ae87ffb",
			"622ce49333fe213ced352b6d",
			"622ce49319dbc3c9f4c2086f",
			"622ce4937687cab096722650",
			"622ce493d33c5cdbd45febe9",
			"622ce4931bcef2e9178e542c",
			"622ce4934d18e78073a37a46",
			"622ce493b8f0db738cf063b2",
			"622ce4936279a81c939cd338",
			"622ce4937f00f1814c35c7ad",
			"622ce4938df4e6680778d657",
			"622ce49329dd07c4fd53e5c7",
			"622ce493d2ae08c7275ee9fd",
			"622ce4934cdc9701f1c0e508",
			"622ce493fe4796d49473f5c4",
			"622ce493a0e95a6675be6338",
			"622ce49368538c5f482ca0e4",
			"622ce4934acfc4dadbe9b3ce",
			"622ce4934721935a5e52b92a",
			"622ce493d4633bfe1450551e",
			"622ce4939daf38efb76a7877",
			"622ce49315ed18f78ef77c27",
			"622ce49363bb05ba93197f95",
			"622ce4930f9ddb51072efee1",
			"622ce493dd2f43d0265b4879",
			"622ce493d34ec1a0ae3447ba",
			"622ce4931c20cfb8c11a4f2d",
			"622ce49353540a4d5476b469",
			"622ce4933e822cf6befdb832",
			"622ce4937e76253ed348e7a3",
			"622ce493c466d6a83fb9127b",
			"622ce493be4bc1faf2f96fd9",
			"622ce493d0e921461012e2dc",
			"622ce493eba251ed296df85d",
			"622ce4930e190d7507ee4f3a"
		]
		}`

	const addresses15 = `{
		"addresses": [
		  "622d0fafb0f939489cbfe777",
		  "622d0faf1f1602be0901acc6",
		  "622d0faf6af4a13297c644e5",
		  "622d0faf13c66a87dd73d2e3",
		  "622d0faf78c7ee3fc59710b4",
		  "622d0faf89e2e955f826dbdd",
		  "622d0faf6f86874cc5ad17cb",
		  "622d0faf082e5c703d8ea5a3",
		  "622d0faf4760a2c1a889ca9a",
		  "622d0faf83ecd60c1f617282",
		  "622d0fafa0269c25634b69c9",
		  "622d0faf4f33450e5bd4c1e4",
		  "622d0faf5b64fee3415fdfe1",
		  "622d0faf252b4778460975e9",
		  "622d0faff01ab45a22911440",
		  "622d0faf9e475436932862d1",
		  "622d0faf6871be58403272fd",
		  "622d0fafa17ae7568f75b572",
		  "622d0faf1397123870a8a0a0",
		  "622d0faf196a96b73481dcb4",
		  "622d0fafb75a796428300684",
		  "622d0fafd913db4d3abe2238",
		  "622d0fafc60d8d7f2bd3ed07",
		  "622d0fafc8e365236da17601",
		  "622d0faf9da97d7a6df7b549",
		  "622d0faf34df8dbd910e9e1e",
		  "622d0fafa40b5fe53f87b620",
		  "622d0faf631b8f35c8b39d6a",
		  "622d0faf62b45836e809c474",
		  "622d0faf0c3bf9d39ab75a5d",
		  "622d0faf677363a414659c8d",
		  "622d0faf0b381cbaa4b62097",
		  "622d0faf5e84b6da7c3bb453",
		  "622d0faf940ef958c2ad8ee7",
		  "622d0faf1350279da74fc9d8",
		  "622d0fafe257c6dc992610a5",
		  "622d0faf95fdbfa5559591d6",
		  "622d0faf824259411c410a7f",
		  "622d0faf06fb92f3e0f0aad7",
		  "622d0faf73958e16f3aff5a3",
		  "622d0faf2d4705bd2ddc667d",
		  "622d0faf5053b67401ca508e",
		  "622d0faf0b310f09db241a7b",
		  "622d0faf1760959b9e14d922",
		  "622d0faf5d3daacc21656af1",
		  "622d0faf65772ad824683e18",
		  "622d0faf994874d72260707b",
		  "622d0faf679ce8f63ce48bb4",
		  "622d0faf3cee4841928a2e3a",
		  "622d0faf24ec516890b12bdf"
		]
	  }`
	const addresses16 = ``
	const addresses17 = ``
	const addresses18 = ``
	const addresses19 = ``
	const addresses20 = ``

	// load addresses
	a := giveaway.ScFuncs.LoadAddresses(ctx)
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses15)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses15)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses15)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses15)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses15)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses15)
	a.Func.TransferIotas(1).Post()
	a.Params.Addresses().SetValue(addresses1)
	a.Func.TransferIotas(1).Post()

	require.NoError(t, ctx.Err)

	// do ruffling
	f := giveaway.ScFuncs.Ruffle(ctx)
	f.Params.NWinners().SetValue(number_of_winners)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)

	// get owner
	g := giveaway.ScFuncs.GetOwner(ctx)
	g.Func.Call()
	require.NoError(t, ctx.Err)
}
