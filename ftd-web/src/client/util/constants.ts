export abstract class Constants {
    // RPC
    static readonly KUSAMA_RPC_URL = 'wss://kusama-rpc.polkadot.io';
    static readonly POLKADOT_RPC_URL = 'wss://rpc.polkadot.io';
    // connection
    static readonly CONNECTION_TIMEOUT_MS = 30000;
    static readonly CONNECTION_RETRY_MS = 5000;
    // UI
    static readonly HASH_TRIM_SIZE = 7;
    static readonly CONTENT_FADE_ANIM_DURATION_MS = 500;
    // format
    static readonly BALANCE_FORMAT_DECIMALS = 4;
    static readonly DECIMAL_SEPARATOR = '.';
    static readonly THOUSANDS_SEPARATOR = ',';
    static readonly MAX_IDENTITY_DISPLAY_LENGTH = 24;
}

export abstract class Kusama {
    static readonly DECIMAL_COUNT = 12;
}

export abstract class Polkadot {
    static readonly DECIMAL_COUNT = 10;
}
