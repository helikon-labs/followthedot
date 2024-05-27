import { trimText, truncateAddress } from '../util/format';
import { Constants } from '../util/constants';

interface Balance {
    free: bigint,
    reserved: bigint,
    frozen: bigint,
}

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
    balance: Balance;
}

function getAccountDisplay(account: Account): string {
    if (account.identity?.display) {
        return trimText(account.identity.display, Constants.MAX_IDENTITY_DISPLAY_LENGTH);
    }
    if (account.superIdentity?.display && account.subIdentity?.subDisplay) {
        const display = `${account.subIdentity.subDisplay} / ${account.superIdentity.display}`;
        return trimText(display, Constants.MAX_IDENTITY_DISPLAY_LENGTH);
    }
    return truncateAddress(account.address);
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

export { Identity, SubIdentity, Account, TransferVolume, GraphData, getAccountDisplay };
