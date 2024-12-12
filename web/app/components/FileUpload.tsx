"use client";

import { useState, useRef, useEffect } from "react";
import {
    FileUploadDropzone,
    FileUploadList,
    FileUploadRoot,
} from "@/components/ui/file-button";

import { broadcastPayloadTx, broadcastProofTx, getNetworkRpcUrl, setupCosmos, uint8ArrayToBase64 } from "hyle-js";
import { network } from "../utils/network";
import { ensureContractsRegistered } from "../utils/cosmos";

type FileUploadResponse = {
    result: boolean;
    success: boolean;
    proof: Uint8Array;
    tx_hash: string;
}

export default function FileUpload() {
    const [uploadStatus, setUploadStatus] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        setupHyle();
    }, []);

    const setupHyle = async () => {
        await setupCosmos(getNetworkRpcUrl(network)!);
        await ensureContractsRegistered();
        setIsLoading(false);
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

            const publishPayload = await broadcastPayloadTx("", [
                {
                    contractName: "sp1_residency",
                    data: "Portugal",
                },
            ]);

            formData.append("tx_hash", publishPayload.transactionHash);

            const response = await fetch("http://localhost:8080/upload", {
                method: "POST",
                body: formData,
            });

            if (response.ok) {
                const data: FileUploadResponse = await response.json();
                console.log("Server response:", data);

                try {
                    if (!isLoading) {
                        console.log("Proof data:", {
                            txHash: data.tx_hash,
                            proofLength: data.proof.length,
                            proof: uint8ArrayToBase64(data.proof).substring(0, 50) + "..."
                        });
                        
                        const result = await broadcastProofTx(
                            data.tx_hash,
                            0,
                            "sp1_residency", 
                            uint8ArrayToBase64(data.proof)
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