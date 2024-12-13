import { checkContractExists, registerContract } from "hyle-js";
import { network } from "./network";

async function waitForServer(port: number, maxRetries = 5, delayMs = 1000): Promise<boolean> {
    for (let i = 0; i < maxRetries; i++) {
        try {
            await fetch(`http://localhost:${port}/verification-key`);
            return true;
        } catch (error) {
            if (i === maxRetries - 1) return false;
            await new Promise(resolve => setTimeout(resolve, delayMs));
        }
    }
    return false;
}

interface VerificationKeyResponse {
    verification_key: string;
}

export async function ensureContractsRegistered(proverPort: number) {
    const exists = await checkContractExists(network, "sp1_residency");
    if (!exists) {
        // Wait for server to be ready
        const serverReady = await waitForServer(proverPort);
        if (!serverReady) {
            throw new Error("Prover server failed to start");
        }

        // Get the verification key
        const response = await fetch(`http://localhost:${proverPort}/verification-key`);
        if (!response.ok) {
            throw new Error("Failed to fetch verification key");
        }
        
        const data: VerificationKeyResponse = await response.json();
        const verificationKey = new Uint8Array(
            Buffer.from(data.verification_key, 'hex')
        );

        await registerContract(
            network,
            "sp1",
            "sp1_residency",
            verificationKey,
            new Uint8Array([0, 0, 0, 0])
        );
    }
}