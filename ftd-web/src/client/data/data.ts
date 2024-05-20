import { GraphData } from '../model/ftd-model';

const DATA: GraphData = {
    accounts: [
        {
            address: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            identity: {
                address: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
                display: '🏔 HELIKON 🏔',
                isConfirmed: true,
                isInvalid: false,
            },
            subIdentity: undefined,
            superIdentity: undefined,
            balance: BigInt('17943564783'),
        },
        {
            address: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            identity: undefined,
            subIdentity: {
                address: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
                superAccountId: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
                subDisplay: 'ISTANBUL',
            },
            superIdentity: {
                address: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
                display: '🏔 HELIKON 🏔',
                isConfirmed: true,
                isInvalid: false,
            },
            balance: BigInt('27640026374856'),
        },
        {
            address: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
            balance: BigInt('12123390036478265'),
        },
    ],
    transferVolumes: [
        {
            id: 1,
            from: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            to: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            count: 12,
            volume: BigInt('12420000000000'),
        },
        {
            id: 2,
            from: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            to: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            count: 3,
            volume: BigInt('224536000000'),
        },
        {
            id: 3,
            from: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            to: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            count: 1,
            volume: BigInt('334325300000000'),
        },
        {
            id: 4,
            from: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            to: '123kFHVth2udmM79sn3RPQ81HukrQWCxA1vmTWkGHSvkR4k1',
            count: 1,
            volume: BigInt('25300000000'),
        },
    ],
};

const DATA_PT2: GraphData = {
    accounts: [
        {
            address: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
            balance: BigInt('541823390036478265'),
        },
        {
            address: '13iTiojfEzSXLprKzvE7Sdmg8gtUD2S2Am2Xv61xrtmDHcvJ',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
            balance: BigInt('3985935463521'),
        },
        {
            address: '14N5GT7YTaDBSsLpfxxtCxNdYfgDofGj5wQSfqC1URKHdT8C',
            identity: undefined,
            subIdentity: undefined,
            superIdentity: undefined,
            balance: BigInt('8941294673827'),
        },
    ],
    transferVolumes: [
        {
            id: 5,
            from: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            to: '13iTiojfEzSXLprKzvE7Sdmg8gtUD2S2Am2Xv61xrtmDHcvJ',
            count: 12,
            volume: BigInt('64720602736748'),
        },
        {
            id: 6,
            from: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            to: '14N5GT7YTaDBSsLpfxxtCxNdYfgDofGj5wQSfqC1URKHdT8C',
            count: 3,
            volume: BigInt('12783047859'),
        },
        {
            id: 7,
            from: '14N5GT7YTaDBSsLpfxxtCxNdYfgDofGj5wQSfqC1URKHdT8C',
            to: '114SUbKCXjmb9czpWTtS3JANSmNRwVa4mmsMrWYpRG1kDH5',
            count: 1,
            volume: BigInt('983674948000'),
        },
        {
            id: 8,
            from: '14N5GT7YTaDBSsLpfxxtCxNdYfgDofGj5wQSfqC1URKHdT8C',
            to: '12WLDL2AXoH3MHr1xj8K4m9rCcRKSWKTUz8A4mX3ah5khJBn',
            count: 1,
            volume: BigInt('143529800047005'),
        },
    ],
};

export { DATA, DATA_PT2 };
