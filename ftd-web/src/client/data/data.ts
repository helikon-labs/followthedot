interface Identity {
    display: string;
    isConfirmed: boolean;
}

interface SubIdentity {
    display: string;
    superAddress: string;
}

interface Account {
    address: string;
    identity: Identity | null;
    subIdentity: SubIdentity | null;
    superIdentity: Identity | null;
}

interface TransferVolume {
    id: number;
    from: string;
    to: string;
    count: number;
    volume: bigint;
}

interface TransferVolumeElement {
    id: number;
    source: string;
    target: string;
    count: number;
    volume: bigint;
    targetDistance: number;
}

interface Data {
    accounts: Account[];
    transferVolumes: TransferVolume[];
}

const DATA: Data = {
    accounts: [
        {
            address: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            identity: null,
            subIdentity: null,
            superIdentity: null,
        },
        {
            address: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            identity: null,
            subIdentity: null,
            superIdentity: null,
        },
        {
            address: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            identity: null,
            subIdentity: null,
            superIdentity: null,
        },
    ],
    transferVolumes: [
        {
            id: 1,
            from: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            to: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            count: 12,
            volume: BigInt(12420000000000),
        },
        {
            id: 2,
            from: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            to: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            count: 3,
            volume: BigInt(224536000000),
        },
        {
            id: 3,
            from: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            to: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            count: 1,
            volume: BigInt(334325300000000),
        },
        {
            id: 4,
            from: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            to: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            count: 1,
            volume: BigInt(25300000000),
        },
    ],
};

export { Account, DATA, Data, TransferVolume, TransferVolumeElement };
