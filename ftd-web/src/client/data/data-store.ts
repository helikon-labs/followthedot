import { ApiPromise, WsProvider } from '@polkadot/api';
import { setAsyncTimeout } from '../util/async-util';
import { EventBus } from '../event/event-bus';
import { Constants } from '../util/constants';
import { FTDEvent } from '../event/event';
import AsyncLock from 'async-lock';

class DataStore {
    private substrateClient?: ApiPromise;
    private readonly eventBus = EventBus.getInstance();
    private readonly lock = new AsyncLock();
    private readonly lockKey = 'block_process';

    async connectSubstrateRPC() {
        try {
            // connection timeout handling
            await setAsyncTimeout((done) => {
                (async () => {
                    this.substrateClient = await ApiPromise.create({
                        provider: new WsProvider(Constants.POLKADOT_RPC_URL),
                    });
                    done(0);
                })();
            }, Constants.CONNECTION_TIMEOUT_MS);
            this.eventBus.dispatch<string>(FTDEvent.SUBSTRATE_API_READY);
        } catch (_error) {
            this.eventBus.dispatch<string>(FTDEvent.SUBSTRATE_API_CONNECTION_TIMED_OUT);
        }
    }

    async disconnectSubstrateClient() {
        if (this.substrateClient) {
            try {
                await this.substrateClient.disconnect();
                this.substrateClient = undefined;
            } catch (error) {
                console.error('Error while disconnecting Substrate client:', error);
            }
        }
    }
}

export { DataStore };
