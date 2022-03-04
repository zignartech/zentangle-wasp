// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

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
	number_of_images := int32(3)
	const number_of_players = 3
	const plays_required_per_image = 1

	// create game
	creator := ctx.NewSoloAgent()
	f := zentangle.ScFuncs.CreateGame(ctx.Sign(creator))
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(number_of_images)
	f.Params.TagsRequiredPerImage().SetValue(plays_required_per_image)
	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)

	//make plays
	var player [number_of_players]*wasmsolo.SoloAgent
	for i := 0; i < number_of_players; i++ {
		player[i] = ctx.NewSoloAgent()
		RequestPlay := zentangle.ScFuncs.RequestPlay(ctx.Sign(player[i]))
		require.NoError(t, ctx.Err)

		SendTags := zentangle.ScFuncs.SendTags(ctx.Sign(player[i]))
		SendTags.Params.InputJson().SetValue(`{
			"x": [50, 200, 500, 150, 300, 400, 600, 700, 800, 900, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000], 
			"y": [1900, 1800, 1700, 1600, 1500, 1400, 1300, 1200, 1100, 1000, 900, 800, 700, 600, 500, 100, 250, 550, 400, 50], 
			"h": [150, 50, 50, 150, 50, 50, 150, 50, 50, 150, 50, 50, 150, 50, 50, 150, 50, 50, 500, 80], 
			"w": [200, 50, 55, 15, 200, 50, 55, 15, 200, 50, 55, 15, 200, 50, 55, 15, 200, 50, 55, 15], 
			"boost": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1]}`)

		for j := 0; int32(j) < (plays_required_per_image * number_of_images / number_of_players); j++ {
			RequestPlay.Func.TransferIotas(1000 + int64(i)).Post()
			SendTags.Func.TransferIotas(1).Post()
		}

		require.NoError(t, ctx.Err)
	}

	getPlayerInfo := zentangle.ScFuncs.GetPlayerInfo(ctx)
	getPlayerInfo.Params.PlayerAddress().SetValue(player[2].ScAddress().String())
	getPlayerInfo.Func.Call()

	getPlayerBets := zentangle.ScFuncs.GetPlayerBets(ctx)
	getPlayerBets.Func.Call()

	GetPlaysPerImage := zentangle.ScFuncs.GetPlaysPerImage(ctx)
	GetPlaysPerImage.Params.ImageId().SetValue(0)
	GetPlaysPerImage.Func.Call()

	// End game
	EndGame := zentangle.ScFuncs.EndGame(ctx.Sign(creator))
	EndGame.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)

	getResults := zentangle.ScFuncs.GetResults(ctx)
	for i := 0; int32(i) < number_of_images; i++ {
		getResults.Params.ImageId().SetValue(int32(i))
		getResults.Func.Call()
	}
}

/*
func TestPlay(t *testing.T) {
	numberOfImages := int32(22)

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
