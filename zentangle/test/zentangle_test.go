// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"fmt"
	"testing"
	"time"

	"github.com/iotaledger/wasp/packages/iscp/colored"
	"github.com/iotaledger/wasp/packages/vm/wasmsolo"
	"github.com/iotaledger/wasp/zentangle/go/zentangle"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, zentangle.ScName, zentangle.OnLoad)
	require.NoError(t, ctx.ContractExists(zentangle.ScName))
}

func TestPlay2(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, zentangle.ScName, zentangle.OnLoad)
	number_of_images := uint32(10)
	const number_of_players = 10
	const plays_required_per_image = 10

	// create game
	creator := ctx.NewSoloAgent()
	f := zentangle.ScFuncs.CreateGame(ctx.Sign(creator))
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(number_of_images)
	f.Params.TagsRequiredPerImage().SetValue(plays_required_per_image)
	f.Func.TransferIotas(400000).Post()
	require.NoError(t, ctx.Err)

	//make plays
	var player [number_of_players]*wasmsolo.SoloAgent
	for i := 0; i < number_of_players; i++ {
		player[i] = ctx.NewSoloAgent()
		RequestPlay := zentangle.ScFuncs.RequestPlay(ctx.Sign(player[i]))
		require.NoError(t, ctx.Err)

		x := 50.4518671871878
		y := 39.46231654
		h := 10.1465456
		w := 11.41564515765165
		SendTags := zentangle.ScFuncs.SendTags(ctx.Sign(player[i]))
		SendTags.Params.InputJson().SetValue(`{
			"x": [` + fmt.Sprintf("%v", x+float64(i)) + `, ` + fmt.Sprintf("%v", 2*x+float64(i)) + `, ` + fmt.Sprintf("%v", 3*x+float64(i)) + `],
			"y": [` + fmt.Sprintf("%v", y+float64(i)) + `, ` + fmt.Sprintf("%v", 2*y+float64(i)) + `, ` + fmt.Sprintf("%v", 3*y+float64(i)) + `],
			"h": [` + fmt.Sprintf("%v", h+float64(i)) + `, ` + fmt.Sprintf("%v", 2*h+float64(i)) + `, ` + fmt.Sprintf("%v", 3*h+float64(i)) + `],
			"w": [` + fmt.Sprintf("%v", w+float64(i)) + `, ` + fmt.Sprintf("%v", 2*w+float64(i)) + `, ` + fmt.Sprintf("%v", 3*w+float64(i)) + `],
			"boost": [1, 1, 1]
		}`)

		RequestPlay.Func.TransferIotas(1).Post()
		for j := 0; uint32(j) < (plays_required_per_image * number_of_images / number_of_players); j++ {
			SendTags.Func.TransferIotas(1000).Post()
		}

		require.NoError(t, ctx.Err)
	}

	getPlayerInfo := zentangle.ScFuncs.GetPlayerInfo(ctx)
	getPlayerInfo.Params.PlayerAddress().SetValue(player[0].ScAddress().String())
	getPlayerInfo.Func.Call()

	getPlayerBets := zentangle.ScFuncs.GetPlayerBets(ctx)
	getPlayerBets.Func.Call()

	ctx.WaitForPendingRequests(5, 8*time.Second)

	/*for i := 0; i < int(number_of_images); i++ {
		GetPlaysPerImage := zentangle.ScFuncs.GetPlaysPerImage(ctx)
		GetPlaysPerImage.Params.ImageId().SetValue(uint32(i))
		GetPlaysPerImage.Func.Call()
	}*/

	// End game
	/*EndGame := zentangle.ScFuncs.EndGame(ctx.Sign(creator))
	EndGame.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)*/

	balances := 0
	for i := 0; i < number_of_players; i++ {
		balances += int(player[i].Env.GetAddressBalance(ctx.Chain.ChainID.AliasAddress, colored.IOTA))
	}
	getResults := zentangle.ScFuncs.GetResults(ctx)
	for i := 0; uint32(i) < number_of_images; i++ {
		getResults.Params.ImageId().SetValue(uint32(i))
		getResults.Func.Call()
	}
}

/*
func TestPlay(t *testing.T) {
	numberOfImages := uint32(22)

	ctx := wasmsolo.NewSoloContext(t, zentangle.ScName, zentangle.OnLoad)

	// create game
	f := zentangle.ScFuncs.CreateGame(ctx)
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(numberOfImages)
	f.Params.TagsRequiredPerImage().SetValue(4)
	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)

	RequestPlay(t, ctx)
	SendTags(t, ctx)
	RequestPlay(t, ctx)
	SendTags(t, ctx)
	RequestPlay(t, ctx)

	// End game
	v := zentangle.ScFuncs.EndGame(ctx)
	v.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)

	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)

	RequestPlay(t, ctx)
	SendTags(t, ctx)
	RequestPlay(t, ctx)
	SendTags(t, ctx)
	RequestPlay(t, ctx)

}

func RequestPlay(t *testing.T, _ctx *wasmsolo.SoloContext) {
	ctx := _ctx

	f := zentangle.ScFuncs.RequestPlay(ctx)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}

func SendTags(t *testing.T, _ctx *wasmsolo.SoloContext) {
	ctx := _ctx

	f := zentangle.ScFuncs.SendTags(ctx)
	f.Params.InputJson().SetValue(`{"x": [50, 200, 500], "y": [100, 250, 550], "h": [150, 50, 50], "w": [200, 50, 55], "boost": [1, 1, 1]}`)

	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}
*/
