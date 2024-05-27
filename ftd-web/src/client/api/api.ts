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
        return 'https://' + this.host + ':' + this.port + '/';
    }

    async searchAccount(query: string): Promise<Account[]> {
        await sleep(1000);
        const result = [];
        result.push(...DATA.accounts);
        return result;
        /*
        return await (
            await fetch(
                this.getBasePath() + 'account?' +
                new URLSearchParams({ q: query }).toString(),
                {
                    method: 'GET',
                    headers: {},
                }
            )
        ).json();
         */
    }

    async getAccount(address: string): Promise<GraphData> {
        await sleep(1000);
        return DATA;
    }
}

export { API };
