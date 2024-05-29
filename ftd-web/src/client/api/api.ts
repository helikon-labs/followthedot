import { Account, GraphData } from '../model/ftd-model';

class API {
    private readonly host: string;
    private readonly port: number;

    constructor(host: string, port: number) {
        this.host = host;
        this.port = port;
    }

    private getBasePath(): string {
        return 'https://' + this.host + ':' + this.port;
    }

    private bigintReviver(key: string, value: any): any {
        if (key === 'volume' || key === 'free' || key === 'reserved' || key === 'frozen') {
            return BigInt(value);
        }
        return value;
    }

    async searchAccount(query: string): Promise<Account[]> {
        return await (
            await fetch(
                this.getBasePath() + '/account?' + new URLSearchParams({ query: query }).toString(),
                {
                    method: 'GET',
                    headers: {},
                },
            )
        ).json();
    }

    async getAccountGraph(address: string): Promise<GraphData> {
        const jsonString = await (
            await fetch(
                `${this.getBasePath()}/account/${address}/graph`,
                {
                    method: 'GET',
                    headers: {},
                },
            )
        ).text();
        return JSON.parse(jsonString, this.bigintReviver)
    }
}

export { API };
