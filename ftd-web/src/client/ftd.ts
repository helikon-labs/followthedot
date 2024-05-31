import { UI } from './ui/ui';
import { Network, POLKADOT } from './model/substrate/network';
import { DataStore } from './data/data-store';
import { EventBus } from './event/event-bus';
import { FTDEvent } from './event/event';
import { Constants } from './util/constants';

class FTD {
    private readonly ui: UI;
    private readonly dataStore: DataStore;
    private readonly eventBus = EventBus.getInstance();
    private network: Network = POLKADOT;

    constructor() {
        this.ui = new UI(this.network);
        this.dataStore = new DataStore();
        // substrate api events
        this.eventBus.register(FTDEvent.SUBSTRATE_API_READY, () => {
            // no-op
        });
        this.eventBus.register(FTDEvent.SUBSTRATE_API_CONNECTION_TIMED_OUT, () => {
            this.onSubstrateAPITimedOut();
        });
    }

    async init() {
        this.ui.init();
    }

    async connect() {
        this.dataStore.connectSubstrateRPC();
    }

    onSubstrateAPITimedOut() {
        setTimeout(() => {
            this.dataStore.connectSubstrateRPC();
        }, Constants.CONNECTION_RETRY_MS);
    }
}

export { FTD };
