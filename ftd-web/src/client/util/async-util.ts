const setAsyncTimeout = (callback: (value: (value: unknown) => void) => void, timeout = 0) =>
    new Promise((resolve, reject) => {
        callback(resolve);
        setTimeout(() => reject('Request is taking too long to response'), timeout);
    });

export { setAsyncTimeout };
