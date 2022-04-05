package main

import (
	"fmt"
	"log"

	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmclient"
	gecko "github.com/superoo7/go-gecko/v3"
)

func main() {

	///////////////////////// GET MIOTA PRICE /////////////////////////

	cg := gecko.NewClient(nil)

	ids := []string{"iota"}
	vc := []string{"usd"}
	sp, err := cg.SimplePrice(ids, vc)
	if err != nil {
		log.Fatal(err)
	}
	miota := (*sp)["iota"]["usd"]
	fmt.Println(fmt.Sprintf("MIOTA is worth %f usd", miota))

	/////////////////////// DO TRANSACTION TO SC ///////////////////////

	keypair := wasmclient.SeedToKeyPair("anyaernyaenryaembenboimvewmaiN BO", 0)

	client := wasmclient.NewServiceClient("https://test-dcsc.zentangle.io/", " ")

	service, err := NewPriceoracleService(client, "gLruuKrc7BNUapPi95MMCLguaAqKjM6sQPjw95yo5iSV")
	if err != nil {
		log.Fatal(err)
	}

	SetMiotaPrice := service.SetMiotaPrice()
	SetMiotaPrice.Price(int64(miota * 1000000))
	SetMiotaPrice.Sign(keypair)
	SetMiotaPrice.Post()

}
