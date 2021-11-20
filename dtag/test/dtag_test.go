package test

import (
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

	// create game
	creator := ctx.NewSoloAgent()
	f := dtag.ScFuncs.CreateGame(ctx.Sign(creator))
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(3)
	f.Params.TagsRequiredPerImage().SetValue(4)
	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)

	//make plays
	var player [10]*wasmsolo.SoloAgent
	for i := 0; i < 10; i++ {
		player[i] = ctx.NewSoloAgent()
		RequestPlay := dtag.ScFuncs.RequestPlay(ctx.Sign(player[i]))
		RequestPlay.Func.TransferIotas(9999).Post()
		require.NoError(t, ctx.Err)

		SendTags := dtag.ScFuncs.SendTags(ctx.Sign(player[i]))
		SendTags.Params.X().SetValue(50 + int64(i))
		SendTags.Params.Y().SetValue(100 + int64(i))
		SendTags.Params.H().SetValue(150 + int64(i))
		SendTags.Params.W().SetValue(200 + int64(i))
		SendTags.Func.TransferIotas(1).Post()
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
	getResults.Params.ImageId().SetValue(0)
	getResults.Func.Call()
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
	f.Params.X().SetValue(x)
	f.Params.Y().SetValue(y)
	f.Params.H().SetValue(h)
	f.Params.W().SetValue(w)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}
