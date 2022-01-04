package test

import (
	"fmt"
	"testing"

	"github.com/iotaledger/wasp/dtag/go/dtag"
	"github.com/iotaledger/wasp/packages/vm/wasmsolo"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)
	require.NoError(t, ctx.ContractExists(dtag.ScName))
}

func TestPlay2(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)
	number_of_images := int32(30)
	const number_of_players = 30
	const tags_required_per_image = 2

	// create game
	creator := ctx.NewSoloAgent()
	f := dtag.ScFuncs.CreateGame(ctx.Sign(creator))
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(number_of_images)
	f.Params.TagsRequiredPerImage().SetValue(tags_required_per_image)
	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)

	//make plays
	var player [number_of_players]*wasmsolo.SoloAgent
	for i := 0; i < number_of_players; i++ {
		player[i] = ctx.NewSoloAgent()
		RequestPlay := dtag.ScFuncs.RequestPlay(ctx.Sign(player[i]))
		require.NoError(t, ctx.Err)

		SendTags := dtag.ScFuncs.SendTags(ctx.Sign(player[i]))
		SendTags.Params.X().SetValue(fmt.Sprint(50+i) + " " + fmt.Sprint(200+i))
		SendTags.Params.Y().SetValue(fmt.Sprint(100+i) + " " + fmt.Sprint(250+i))
		SendTags.Params.H().SetValue(fmt.Sprint(150+i) + " " + fmt.Sprint(50+i))
		SendTags.Params.W().SetValue(fmt.Sprint(200+i) + " " + fmt.Sprint(50+i))
		SendTags.Params.Boost().SetValue(fmt.Sprint(1+(i%3)) + " " + fmt.Sprint(1))

		for j := 0; int32(j) < (tags_required_per_image * number_of_images / number_of_players); j++ {
			RequestPlay.Func.TransferIotas(1000 + int64(i)).Post()
			SendTags.Func.TransferIotas(1).Post()
		}

		require.NoError(t, ctx.Err)
	}

	getPlayerBets := dtag.ScFuncs.GetPlayerBets(ctx)
	getPlayerBets.Func.Call()

	GetPlaysPerImage := dtag.ScFuncs.GetPlaysPerImage(ctx)
	GetPlaysPerImage.Params.ImageId().SetValue(0)
	GetPlaysPerImage.Func.Call()

	// End game
	EndGame := dtag.ScFuncs.EndGame(ctx.Sign(creator))
	EndGame.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)

	getResults := dtag.ScFuncs.GetResults(ctx)
	for i := 0; int32(i) < number_of_images; i++ {
		getResults.Params.ImageId().SetValue(int32(i))
		getResults.Func.Call()
	}
}

func TestPlay(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)

	// create game
	f := dtag.ScFuncs.CreateGame(ctx)
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(4)
	f.Params.TagsRequiredPerImage().SetValue(4)
	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)

	//make plays
	RequestPlay(t, ctx)
	SendTags(t, ctx, 50, 100, 150, 200)
	RequestPlay(t, ctx)
	SendTags(t, ctx, 50, 100, 150, 200)
	RequestPlay(t, ctx)
	SendTags(t, ctx, 50, 100, 150, 200)
	RequestPlay(t, ctx)
	SendTags(t, ctx, 50, 100, 150, 200)

	getPlayerBets := dtag.ScFuncs.GetPlayerBets(ctx)
	getPlayerBets.Func.Call()

	// End game
	v := dtag.ScFuncs.EndGame(ctx)
	v.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}

func RequestPlay(t *testing.T, _ctx *wasmsolo.SoloContext) {
	ctx := _ctx

	f := dtag.ScFuncs.RequestPlay(ctx)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}

func SendTags(t *testing.T, _ctx *wasmsolo.SoloContext, x int64, y int64, h int64, w int64) {
	ctx := _ctx

	f := dtag.ScFuncs.SendTags(ctx)
	f.Params.X().SetValue(fmt.Sprint(x))
	f.Params.Y().SetValue(fmt.Sprint(y))
	f.Params.H().SetValue(fmt.Sprint(h))
	f.Params.W().SetValue(fmt.Sprint(w))
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}
