import { checkContractExists, hexToUint8Array, registerContract } from "hyle-js";
import { network } from "./network";

export async function ensureContractsRegistered() {
    const exists = await checkContractExists(network, "sp1_residency");
    if (!exists) {
        const b64vKey = "0x003f11abc87db2d83462a022692a6c10fec3999b54c63f955400b22d885a0ead";  // TODO: process.env.SP1_PUBLIC_KEY;
        const vKey = hexToUint8Array(b64vKey);
        await registerContract(
            "sp1",
            "sp1_residency",
            vKey,
            new Uint8Array([0, 0, 0, 0]), // TODO: add Nonce in digest?
        );
    }
}