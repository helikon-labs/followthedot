import { Constants } from '../../util/constants';

/**
 * Substrate network.
 */
interface Network {
    readonly id: string;
    readonly display: string;
    readonly tokenTicker: string;
    readonly tokenDecimals: number;
    readonly ss58Prefix: number;
    readonly logo: string;
    readonly rpcURL: string;
    readonly apiHost: string;
    readonly apiPort: number;
}

const KUSAMA: Network = {
    id: 'kusama',
    display: 'Kusama',
    tokenTicker: 'KSM',
    tokenDecimals: 12,
    ss58Prefix: 2,
    logo: 'kusama.svg',
    rpcURL: Constants.KUSAMA_RPC_URL,
    apiHost: 'kusama.api.followthedot.live',
    apiPort: 11210,
};

const POLKADOT: Network = {
    id: 'polkadot',
    display: 'Polkadot',
    tokenTicker: 'DOT',
    tokenDecimals: 10,
    ss58Prefix: 0,
    logo: 'polkadot-circle.svg',
    rpcURL: Constants.POLKADOT_RPC_URL,
    apiHost: 'polkadot.api.followthedot.live',
    apiPort: 11200,
};

export { Network, KUSAMA, POLKADOT };
