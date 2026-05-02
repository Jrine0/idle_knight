type FreighterApi = {
    isConnected: () => Promise<{ isConnected: boolean }>;
    requestAccess: () => Promise<{ error?: string }>;
    getAddress: () => Promise<{ address?: string; error?: string }>;
};

const getFreighterApi = (): FreighterApi => {
    const api = (globalThis as any)?.freighterApi;
    if (!api) throw new Error('Freighter API not available in this environment');
    return api as FreighterApi;
};

export async function connectFreighterWallet(): Promise<string> {
    const api = getFreighterApi();
    const connected = await api.isConnected();
    if (!connected.isConnected) {
        const access = await api.requestAccess();
        if (access.error) throw new Error(access.error);
    }
    const account = await api.getAddress();
    if (account.error || !account.address)
        throw new Error(account.error || 'Unable to load wallet address');
    return account.address;
}
