import { GraphData } from '../model/ftd-model';

const DATA: GraphData = {
    accounts: [
        {
            address: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
        },
        {
            address: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
        },
        {
            address: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
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

export { DATA };
