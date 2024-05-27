const setAsyncTimeout = (callback: (value: (value: unknown) => void) => void, timeout = 0) =>
    new Promise((resolve, reject) => {
        callback(resolve);
        setTimeout(() => reject('Request is taking too long to response'), timeout);
    });

async function sleep(ms: number): Promise<void> {
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve();
        }, ms);
    });
}

export { setAsyncTimeout, sleep };
