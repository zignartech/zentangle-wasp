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
