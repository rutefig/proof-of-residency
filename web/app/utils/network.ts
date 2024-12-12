export const network = process.env.NODE_ENV === "production" ? "devnet" : "localhost";

export const getNetworkApiUrl = () => {
    return {
        localhost: "http://localhost:1317",
        devnet: "https://api.devnet.hyle.eu",
    }[network];
};

export const getNetworkRpcUrl = () => {
    return {
        localhost: "http://localhost:26657",
        devnet: "https://rpc.devnet.hyle.eu",
    }[network];
};

export const getNetworkWebsocketUrl = () => {
    return {
        localhost: "ws://localhost:26657/websocket",
        devnet: "wss://rpc.devnet.hyle.eu/websocket",
    }[network];
};

export const getSp1ProverUrl = () => {
    return {
        localhost: "http://localhost:8080",
        devnet: "https://vibe.hyle.eu/sp1prover",
    }[network];
};

export const getNoirProverUrl = () => {
    return {
        localhost: "http://localhost:3001",
        devnet: "https://vibe.hyle.eu/noirprover",
    }[network];
};

export const getRpId = () => {
    return {
        localhost: "localhost",
        devnet: "vibe.hyle.eu",
    }[network];
};
