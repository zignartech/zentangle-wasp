package chain

import (
	"github.com/iotaledger/wasp/packages/iscp"
	"github.com/iotaledger/wasp/tools/wasp-cli/config"
	"github.com/iotaledger/wasp/tools/wasp-cli/log"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var chainAlias string

func GetChainAlias() string {
	if chainAlias == "" {
		chainAlias = viper.GetString("chain")
	}
	if chainAlias == "" {
		log.Fatalf("No current chain. Call `chain deploy --chain=<alias>` or `set chain <alias>`")
	}
	return chainAlias
}

func SetCurrentChain(chainAlias string) {
	config.Set("chain", chainAlias)
}

func initAliasFlags(chainCmd *cobra.Command) {
	chainCmd.PersistentFlags().StringVarP(&chainAlias, "chain", "a", "", "chain alias")
}

func AddChainAlias(chainAlias, id string) {
	config.Set("chains."+chainAlias, id)
	SetCurrentChain(chainAlias)
}

func GetCurrentChainID() *iscp.ChainID {
	return GetChainFromAlias(GetChainAlias())
}

func GetChainFromAlias(alias string) *iscp.ChainID {
	chid, err := iscp.ChainIDFromBase58(viper.GetString("chains." + alias))
	log.Check(err)
	return chid
}
