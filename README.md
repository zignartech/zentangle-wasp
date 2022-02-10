![Wasp logo](WASP_logo_dark.png)

# Welcome to the Wasp repository!

[Wasp](https://github.com/iotaledger/wasp) is a node software developed by the
[IOTA Foundation](http://iota.org) to run the _IOTA Smart Contract Protocol_
(_ISCP_ in short) on top of the _IOTA Tangle_.  Here's a [high level
introduction](https://blog.iota.org/an-introduction-to-iota-smart-contracts-16ea6f247936)
into ISCP.

The comprehensive overview of design decisions of _IOTA Smart Contracts_ can be found in the
[whitepaper](https://github.com/iotaledger/wasp/raw/master/documentation/ISC_WP_Nov_10_2021.pdf).

## Documentation

The documentation for Wasp and IOTA Smart Contracts can be found on the [IOTA Wiki](https://wiki.iota.org/wasp/overview).

## Contributing

If you want to contribute to this repository, consider posting a [bug
report](https://github.com/iotaledger/wasp/issues/new-issue), feature request
or a [pull request](https://github.com/iotaledger/wasp/pulls/).

Please read [this](documentation/docs/contribute.md) before creating a pull request.

You can also join our [Discord server](https://discord.iota.org/) and ping us
in `#smartcontracts-dev`.


# Data Clustering Smart Contract
The Data Clustering Smart Contract processes the annotation data made by players to calculate the centers of each cluster, identify outliers, and reward players based on the relative accuracy compared to other players making sure that the collective annotation is completed with accuracy. The outlier annotations are discarded and are not eligible for campaign rewards. 

On the other hand, the Data Clustering Smart Contract accepts additional payments from the players that want to increase their bet during the campaigns. At the end of the campaign, the smart contract distributes the sponsored bets and players bets based on the power of two of the relative positions multiplied by the total amount of the player's bet relative to the total betting pot.

The steps followed by system in a normal campaign are as follows:
1. A data scientist creates a campaign uploading the images to annotate including the objective, annotation classes, key words, number of annotators, number of validators and pays to the smart contract for the campaign.
1. The backend creates unique identifiers for the images and sends the campaign details to the smart contract.
1. Once the Data Clustering Smart Contract receives the funds and image list with all the details in place, players can request images through the application frontend to start annotating and betting.
1. The smart contract chooses an image ID randomly and sends it to the frontend, then the frontend pulls the image from the backend and presents it to the player.
1. The player annotates the image and sends the annotation details by requesting the next image to the smart contract. Each image annotation has four dimensions: X coordinate, Y coordinate, Height and Width.
1. After the number of annotators previously defined by the data scientist (sovereign) is reached for an image, the smart contract marks the image as attempted, not presenting it to other players until the campaign is completed.
1. When the campaign is completed the smart contract takes all the annotation data, for every image, and starts to calculate the annotation clusters using the four dimensions that represent each annotation. 
1. The smart contract discards small clusters or outliers, and calculates the distribution among valid annotations.
1. The Data Clustering Smart Contract ranks players based on shortest distance to the center of each annotated image, considering only the best image of each player. Then, take the total number of players plus one minus the position to the power of two to calculate the positions points for each player.
    * Position Points = (number of players + 1 - player’s position) ^ 2
1. The smart contract uses each player's bet considering sponsored bet amount and the total additional bet amount to multiply this with the position points.
    * Player’s Bet = (Sponsored Bet amount + all Total Additional Bet amount)
    * Players points = ( Player’s Bet * Position Points)
1. Valid play’s that haven’t been marked as outliers, are used by the smart contract to divide each Player’s Points by All Players Aggregated Points and multiplies this division with the total Betting Pot to determine the reward for each player.
    * Reward = (Player Points / All Players Aggregated Points ) * Betting Pot
1. Finally, the smart contract starts to fund the rewards directly to the registered wallets of players. The players with most accuracy get exponentially better payouts, and proportionally to their cumulative bets.
1. If all annotations made by a player are marked as outliers, then the betting funds go to ZenTangle Wallet, but if at least one image is not marked as outlier, then the player is rewarded depending on how that image annotation is ranked.

# Data Clustering Algorithm
The clusterization algorithm used to merge the annotations that are close to each other is called **Agglomerative Hierarchical Clusterization Algorithm**. This algorithm, unlike most of other clusterization algorithms, doesn't take as manual input the amount of clusters to consider. This is an important feature, because we do not know how many annotations there will be. Tho, it does require a manual input, this is the distance at which it will no longer merge two particular clusters.

The  “Agglomerative Hierarchical Clusterization Algorithm“ works as follows: 

1. Every annotation starts as a cluster.
1. Find the two closest clusters.
1. Merge those clusters into one cluster. In our case, we set it’s new coordinates as the average of all points inside both clusters. This is because our clusters normally take a circular shape.
1. Repeat this step until the two closest clusters are further away than the minimum distance set as the minimum inter cluster distance constant. 
1. The algorithm counts the amount of points inside of clusters; if this number is less than the confirmation percentage of players, then the cluster is discarded, and not considered for the results. The confirmation percentage is a constant parameter.

As mentioned before, images are presented randomly by the smart contract and the plays are only accepted from verified players through the frontend. Then, an important assumption is that, at any moment, collectively all players will not set up more than one annotation at the same time for the same object in the image. This allows the algorithm to iterate until the number of clusters matches the objects to annotate compared to other players, ensuring that some player's repeated annotations for the same object can be discarded. These features allow the algorithm to work for images with low or moderate agglomeration of objects.

The algorithm decides if a cluster is valid using a percentage of players that annotate an object near the same coordinates, this percentage is known as **confirmation percentage**. But the algorithm also can decide on the tags. If there are two or more tags from a player within the same cluster, then this point is left apart with another in similar conditions, and then only over these points, the algorithm is applied again, looking for new possible clusters using the **confirmation percentage**. This could happen if two objects are to close one from the other, and two tags of the same player are close enough.
