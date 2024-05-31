import { trimText, truncateAddress } from '../util/format';
import { Constants } from '../util/constants';

interface Balance {
    free: bigint;
    reserved: bigint;
    frozen: bigint;
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

interface SubscanParentAccountDisplay {
    address: string;
    display?: string;
    subSymbol?: string;
    identity?: boolean;
}

interface SubscanMerkleScienceAccountInfo {
    addressType: string;
    tagType?: string;
    tagSubtype?: string;
    tagName: string;
}

interface SubscanAccountDisplay {
    address: string;
    accountIndex?: string;
    display?: string;
    identity?: boolean;
    parent?: SubscanParentAccountDisplay;
    merkle?: SubscanMerkleScienceAccountInfo;
}

interface SubscanAccount {
    address: string;
    display?: string;
    accountDisplay?: SubscanAccountDisplay;
}

interface Account {
    address: string;
    identity?: Identity;
    subIdentity?: SubIdentity;
    superIdentity?: Identity;
    balance: Balance;
    subscanAccount?: SubscanAccount;
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

function getAccountSubscanDisplay(account: Account): string | undefined {
    const merkle = account.subscanAccount?.accountDisplay?.merkle;
    if (merkle) {
        return merkle.tagName;
    } else if (account.subscanAccount?.accountDisplay?.accountIndex) {
        return account.subscanAccount?.accountDisplay?.accountIndex;
    } else if (account.subscanAccount?.display) {
        return account.subscanAccount?.display;
    } else if (account.subscanAccount?.accountDisplay?.display) {
        return account.subscanAccount?.accountDisplay?.display;
    }
    return undefined;
}

function getAccountDisplay(account: Account): string {
    if (account.identity?.display) {
        return trimText(account.identity.display, Constants.MAX_IDENTITY_DISPLAY_LENGTH);
    }
    if (account.superIdentity?.display && account.subIdentity?.subDisplay) {
        const display = `${account.superIdentity.display} / ${account.subIdentity.subDisplay}`;
        return trimText(display, Constants.MAX_IDENTITY_DISPLAY_LENGTH);
    }
    return truncateAddress(account.address);
}

function getAccountConfirmedIcon(account: Account): string | undefined {
    function getIdentityConfirmedIcon(identity: Identity, isParent: boolean): string | undefined {
        if (identity.isInvalid) {
            return `./img/icon/${isParent ? 'parent-' : ''}id-invalid-icon.svg`;
        }
        if (identity.isConfirmed) {
            return `./img/icon/${isParent ? 'parent-' : ''}id-confirmed-icon.svg`;
        }
        return `./img/icon/${isParent ? 'parent-' : ''}id-unconfirmed-icon.svg`;
    }

    if (account.superIdentity) {
        return getIdentityConfirmedIcon(account.superIdentity, true);
    }
    if (account.identity) {
        return getIdentityConfirmedIcon(account.identity, false);
    }
    return undefined;
}

export {
    Identity,
    SubIdentity,
    Account,
    TransferVolume,
    GraphData,
    getAccountDisplay,
    getAccountSubscanDisplay,
    getAccountConfirmedIcon,
};
