"use client";

import { useState, useRef, useEffect } from "react";
import {
    FileUploadDropzone,
    FileUploadList,
    FileUploadRoot,
} from "@/components/ui/file-button";

import { broadcastBlobTx, broadcastProofTx } from "hyle-js";
import { network } from "../utils/network";
import { ensureContractsRegistered } from "../utils/cosmos";
import { cleanupProverSession, createProverSession } from "@/api";

type FileUploadResponse = {
    result: boolean;
    success: boolean;
    proof: Uint8Array;
    tx_hash: string;
}

export default function FileUpload() {
    const [uploadStatus, setUploadStatus] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [sessionId, setSessionId] = useState<string | null>(null);
    const [proverPort, setProverPort] = useState<number | null>(null);
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        setupSession();

        // Cleanup session on unmount
        return () => {
            if (sessionId) {
                cleanupProverSession(sessionId).catch(console.error);
            }
        };
    }, []);

    const setupSession = async () => {
        try {
            const { session_id, prover_port } = await createProverSession();
            setSessionId(session_id);
            await setProverPort(prover_port);
            await ensureContractsRegistered(prover_port);
        } catch (error) {
            console.error('Failed to setup session:', error);
        } finally {
            setIsLoading(false);
        }
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        const file = inputRef.current?.files?.[0];
        if (!file) {
            setUploadStatus("Error: No file selected");
            return;
        }

        const formData = new FormData();
        formData.append("file", file);
        formData.append("country", "Portugal");



        try {
            setUploadStatus("Uploading...");

            const txHash = await broadcastBlobTx(network, {
                identity: "", // TODO
                blobs: [
                    {
                        contractName: "sp1_residency",
                        data: Array.from(new TextEncoder().encode("Portugal")),
                    },
                ]
            });

            formData.append("tx_hash", txHash);

            const response = await fetch(`http://localhost:${proverPort}/upload`, {
                method: "POST",
                body: formData,
            });
            const proofBuffer = await response.arrayBuffer();
            const proofBytes = new Uint8Array(proofBuffer);

            if (response.ok) {

                try {
                    if (!isLoading) { 
                        const result = await broadcastProofTx(
                            network,
                            txHash,
                            0,
                            "sp1_residency",
                            proofBytes
                        );
                        console.log("Proof broadcasted:", result);
                    }
                } catch (error) {
                    console.error("Broadcast error:", error);
                    setUploadStatus("Error broadcasting proof");
                }

                setUploadStatus("File processed successfully!");
            } else {
                setUploadStatus("Error processing file");
            }
        } catch (error) {
            console.error("Upload error:", error);
            setUploadStatus("Error uploading file");
        }
    };

    return (
        <div className="flex flex-col gap-8 text-center sm:text-center w-full">
            <div className="space-y-6">
                <h3 className="text-xl font-semibold text-gray-800">
                    Upload a Portuguese utility bill to verify residence
                </h3>
            </div>

            <form onSubmit={handleSubmit} className="w-full">
                <FileUploadRoot
                    alignItems="stretch"
                    maxFiles={1}
                    accept=".pdf"
                    ref={inputRef}
                >
                    <FileUploadDropzone
                        label="Drag and drop here to upload"
                        description=".pdf files up to 5MB"
                    />
                    <FileUploadList />
                    <button
                        type="submit"
                        className="mt-4 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 transition-colors"
                    >
                        Submit
                    </button>
                </FileUploadRoot>
            </form>

            {uploadStatus && (
                <div className={`text-sm font-medium ${uploadStatus.includes("Error") ? "text-red-500" : "text-green-500"
                    }`}>
                    {uploadStatus}
                </div>
            )}
        </div>
    );
}