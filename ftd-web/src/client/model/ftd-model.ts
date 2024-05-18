interface Identity {
    address: string;
    display?: string;
    email?: string;
    legal?: string;
    riot?: string;
    twitter?: string;
    web?: string;
    isConfirmed: boolean;
    isInvalid: boolean;
}

interface SubIdentity {
    address: string;
    superAccountId: string;
    subDisplay?: string;
}

interface Account {
    address: string;
    identity?: Identity;
    subIdentity?: SubIdentity;
    superIdentity?: Identity;
}

interface TransferVolume {
    id: number;
    from: string;
    to: string;
    count: number;
    volume: bigint;
}

interface GraphData {
    accounts: Account[];
    transferVolumes: TransferVolume[];
}

export { Identity, SubIdentity, Account, TransferVolume, GraphData };
