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

func TestCreateGame(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)

	f := dtag.ScFuncs.CreateGame(ctx)
	f.Params.Description().SetValue("Esto es un test")
	f.Params.NumberOfImages().SetValue(5)
	f.Params.TagsRequiredPerImage().SetValue(4)
	f.Func.TransferIotas(10000).Post()
	require.NoError(t, ctx.Err)
}

func RequestPlay(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)

	f := dtag.ScFuncs.RequestPlay(ctx)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}

func SendTags(t *testing.T, x int64, y int64, h int64, w int64) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)

	f := dtag.ScFuncs.SendTags(ctx)
	f.Params.X().SetValue(x)
	f.Params.Y().SetValue(y)
	f.Params.H().SetValue(h)
	f.Params.W().SetValue(w)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}

func TestPlay(t *testing.T) {

	RequestPlay(t)
	SendTags(t, 50, 100, 150, 200)
	RequestPlay(t)
	SendTags(t, 50, 100, 150, 200)
	RequestPlay(t)
	SendTags(t, 50, 100, 150, 200)
	RequestPlay(t)
	SendTags(t, 50, 100, 150, 200)
}

func TestEndGame(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, dtag.ScName, dtag.OnLoad)

	f := dtag.ScFuncs.EndGame(ctx)
	f.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}
