const config = require('../config.dev');
import { BasicClient, Buffer, IKeyPair, IOnLedger, WalletService } from './wasp_client';
import { createNanoEvents, Emitter, Unsubscribe } from 'nanoevents';
import { HName } from './wasp_client/crypto/hname';

type MessageHandlers = { [key: string]: (index: number) => void };
type ParameterResult = { [key: string]: Buffer };


export interface Events {
  gameStarted: (number_of_images: number, tags_required_per_image: number, reward: number, description: string) => void;
  gameEnded: () => void;
  playRequested: (address: string, amount: number, image_id: number) => void;
  imageTagged: (address: string, plays_per_image: number) => void;
}

export class ViewEntrypoints {
  public static readonly playsPerImage: string = 'playsPerImage';
  public static readonly results: string = 'results';
  public static readonly playerBets: string = 'public static readonly';
}

export class DTagService {
  private readonly scName: string = config.contractName;
  private readonly scHName: string = HName.HashAsString(this.scName);
  private readonly scSendTags: string = 'sendTags';
  private readonly scRequestPlay: string = 'requestPlay';

  private client: BasicClient;
  private walletService: WalletService;
  private webSocket: WebSocket|undefined;
  private emitter: Emitter;

  public chainId: string;
  public static readonly roundLength: number = 60; // in seconds

  constructor(client: BasicClient, chainId: string) {
    this.walletService = new WalletService(client);
    this.client = client;
    this.chainId = chainId;
    this.emitter = createNanoEvents();

    this.connectWebSocket();
  }

  private connectWebSocket(): void {
    const webSocketUrl = config.waspWebSocketUrl.replace('%chainId', this.chainId);
    // eslint-disable-next-line no-console
    console.log(`Connecting to Websocket => ${webSocketUrl}`);
    this.webSocket = new WebSocket(webSocketUrl);
    this.webSocket.addEventListener('message', (x) => this.handleIncomingMessage(x));
    this.webSocket.addEventListener('close', () => setTimeout(this.connectWebSocket.bind(this), 1000));
  }

  private handleVmMessage(message: string[]): void {
    const messageHandlers: MessageHandlers = {
      'game.started': (index) => {
        this.emitter.emit('gameStarted', message[index + 1], message[index + 2], message[index + 3], message[index + 4]);
      },

      'dtag.game.ended': () => {
        this.emitter.emit('gameEnded');
      },

      'play.requested': (index) => {
        this.emitter.emit('playRequested', message[index + 1], message[index + 2], message[index + 3]);
      },

      'dtag.image.tagged': (index) => {
        this.emitter.emit('imageTagged', message[index + 1], message[index + 2]);
      },
    };

    const topicIndex = 3;
    const topic = message[topicIndex];

    if (typeof messageHandlers[topic] != 'undefined') {
      messageHandlers[topic](topicIndex);
    }
  }

  private handleIncomingMessage(message: MessageEvent<string>): void {
    const msg = message.data.toString().split(' ');

    if (msg.length == 0) {
      return;
    }

    if (msg[0] != 'vmmsg') {
      return;
    }

    this.handleVmMessage(msg);
  }


  public async requestPlay(keyPair: IKeyPair, address: string, take: bigint): Promise<ParameterResult> {
    const playRequest: IOnLedger = {
      contract: HName.HashAsNumber(this.scName),
      entrypoint: HName.HashAsNumber(this.scRequestPlay),
      arguments: [],
    };

    const response = await this.walletService.sendOnLedgerRequest(keyPair, address, this.chainId, playRequest, take);
    const resultMap: ParameterResult = {};

    if (response.transaction_id) {
      const key = Buffer.from("transaction_id", 'base64').toString();
      const value = Buffer.from(response.transaction_id, 'base64');

      resultMap[key] = value;
    }

    return resultMap;
  }

  public async sendTags(keyPair: IKeyPair, address: string,  x: number, y: number, h: number, w: number, take: bigint): Promise<void> {
    const tagsSending: IOnLedger = {
      contract: HName.HashAsNumber(this.scName),
      entrypoint: HName.HashAsNumber(this.scSendTags),
      arguments: [{ key: '-x', value: x }, {key: '-y', value: y}, {key: '-h', value: h}, {key: '-w', value: w}],
    };

    await this.walletService.sendOnLedgerRequest(keyPair, address, this.chainId, tagsSending, take);
  }
  


  public async callView(viewName: string): Promise<ParameterResult> {
    const response = await this.client.callView(this.chainId, this.scHName, viewName);
    const resultMap: ParameterResult = {};

    if (response.Items) {
      for (const item of response.Items) {
        const key = Buffer.from(item.Key, 'base64').toString();
        const value = Buffer.from(item.Value, 'base64');

        resultMap[key] = value;
      }
    }

    return resultMap;
  }

  public async getPlaysPerImage(): Promise<number> {
    const response = await this.callView(ViewEntrypoints.playsPerImage);
    const plays_per_image = response[ViewEntrypoints.playsPerImage];

    if (!plays_per_image) {
      throw Error(`Failed to get ${ViewEntrypoints.playsPerImage}`);
    }

    return plays_per_image.readUInt32LE(0);
  }

  public async getResults(): Promise<string> {
    const response = await this.callView(ViewEntrypoints.results);
    const results = response[ViewEntrypoints.results];

    if (!results) {
      throw Error(`Failed to get ${ViewEntrypoints.results}`);
    }

    return results.toString();
  }

  public async getPlayerBets(): Promise<string> {
    const response = await this.callView(ViewEntrypoints.playerBets);
    const playerBets = response[ViewEntrypoints.playerBets];

    if (!playerBets) {
      throw Error(`Failed to get ${ViewEntrypoints.playerBets}`);
    }

    return playerBets.toString();
  }

  public on<E extends keyof Events>(event: E, callback: Events[E]): Unsubscribe {
    return this.emitter.on(event, callback);
  }
}
