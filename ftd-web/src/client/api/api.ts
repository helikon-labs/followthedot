import { Account, GraphData } from '../model/ftd-model';
import { sleep } from '../util/async-util';
import { DATA } from '../data/data';

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
        alert(`${this.getBasePath()}/account/${address}/graph`)
        return await (
            await fetch(
                `${this.getBasePath()}/account/${address}/graph`,
                {
                    method: 'GET',
                    headers: {},
                },
            )
        ).json();
    }
}

export { API };
