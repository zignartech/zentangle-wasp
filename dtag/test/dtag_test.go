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
