import { checkContractExists, registerContract } from "hyle-js";
import { network } from "./network";

// TODO: handle this more gracefully than just trying multiple times
async function waitForServer(port: number, maxRetries = 5, delayMs = 1000): Promise<boolean> {
    for (let i = 0; i < maxRetries; i++) {
        try {
            await fetch(`http://localhost:${port}/elf`);
            return true;
        } catch (error) {
            if (i === maxRetries - 1) return false;
            await new Promise(resolve => setTimeout(resolve, delayMs));
        }
    }
    return false;
}

export async function ensureContractsRegistered(proverPort: number) {
    const exists = await checkContractExists(network, "sp1_residency");
    if (!exists) {

        // Wait for server to be ready
        const serverReady = await waitForServer(proverPort);
        if (!serverReady) {
            throw new Error("Prover server failed to start");
        }

        // Now get the ELF
        const response = await fetch(`http://localhost:${proverPort}/elf`);
        const elfBuffer = await response.arrayBuffer();
        const elfBytes = new Uint8Array(elfBuffer);


        await registerContract(
            network,
            "sp1",
            "sp1_residency",
            elfBytes,
            new Uint8Array([0, 0, 0, 0])
        );

    }
}