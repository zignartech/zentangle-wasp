//1. Import coingecko-api
const CoinGecko = require('coingecko-api');

//2. Initiate the CoinGecko API Client
const CoinGeckoClient = new CoinGecko();

//3. Make calls
var func = async() => {
    let data = await CoinGeckoClient.simple.price({
        ids: ['iota'],
        vs_currencies: ['usd'],
    });
    console.log(data.data.iota.usd);
};

func();