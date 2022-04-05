// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
	"github.com/iotaledger/wasp/priceoracle/go/priceoracle"
	"github.com/stretchr/testify/require"
)

func setupTest(t *testing.T) *wasmsolo.SoloContext {
	*wasmsolo.RsWasm = true
	return wasmsolo.NewSoloContext(t, priceoracle.ScName, priceoracle.OnLoad)
}

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, priceoracle.ScName, priceoracle.OnLoad)
	require.NoError(t, ctx.ContractExists(priceoracle.ScName))
}

func TestPlay2(t *testing.T) {
	ctx := setupTest(t)

	const numer_of_oracles = 5
	owner := ctx.Creator()

	//make oracles
	var oracle [numer_of_oracles]*wasmsolo.SoloAgent
	for i := 0; i < numer_of_oracles; i++ {

		oracle[i] = ctx.NewSoloAgent()
		ApproveOracle := priceoracle.ScFuncs.ApproveOracle(ctx.Sign(owner))
		ApproveOracle.Params.Agentid().SetValue(oracle[i].ScAgentID())
		ApproveOracle.Func.TransferIotas(1).Post()
		require.NoError(t, ctx.Err)
	}

	for i := 0; i < 2; i++ {

		for j := 0; j < numer_of_oracles; j++ {
			setMiotaPrice := priceoracle.ScFuncs.SetMiotaPrice(ctx.Sign(oracle[j]))
			setMiotaPrice.Params.Price().SetValue(90 + int64(2*j))
			setMiotaPrice.Func.TransferIotas(1).Post()
			require.NoError(t, ctx.Err)
		}

		setMiotaPrice := priceoracle.ScFuncs.SetMiotaPrice(ctx.Sign(oracle[0]))
		setMiotaPrice.Params.Price().SetValue(100)
		setMiotaPrice.Func.TransferIotas(1).Post()
		require.NoError(t, ctx.Err)

		getMiotaPrice := priceoracle.ScFuncs.GetMiotaPrice(ctx)
		getMiotaPrice.Func.Call()
		require.NoError(t, ctx.Err)
	}
}
