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
}

const KUSAMA: Network = {
    id: 'kusama',
    display: 'Kusama',
    tokenTicker: 'KSM',
    tokenDecimals: 12,
    ss58Prefix: 2,
    logo: 'kusama.svg',
    rpcURL: Constants.KUSAMA_RPC_URL,
};

const POLKADOT: Network = {
    id: 'polkadot',
    display: 'Polkadot',
    tokenTicker: 'DOT',
    tokenDecimals: 10,
    ss58Prefix: 0,
    logo: 'polkadot-circle.svg',
    rpcURL: Constants.POLKADOT_RPC_URL,
};

export { Network, KUSAMA, POLKADOT };
