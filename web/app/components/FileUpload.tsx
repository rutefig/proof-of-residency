"use client";

import { useState, useRef, useEffect, FormEvent } from "react";
import {
    FileUploadDropzone,
    FileUploadList,
    FileUploadRoot,
} from "@/components/ui/file-button";

import { broadcastBlobTx, broadcastProofTx } from "hyle-js";
import { network } from "../utils/network";
import { ensureContractsRegistered } from "../utils/hyle";
import { cleanupProverSession, createProverSession } from "@/api";
import { ProgressCircleRing, ProgressCircleRoot } from "@/components/ui/progress-circle";

export default function FileUpload() {
    const [uploadState, setUploadState] = useState({
        status: "idle",
        message: "",
        error: "",
        progress: 0
    });
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
            setUploadState(prev => ({ ...prev, status: "initializing", message: "Setting up prover session..." }));
            const { session_id, prover_port } = await createProverSession();
            setSessionId(session_id);
            await setProverPort(prover_port);
            setUploadState(prev => ({ ...prev, status: "initializing", message: "Registering contracts...", progress: 60 }));
            await ensureContractsRegistered(prover_port);
            setUploadState(prev => ({ ...prev, status: "ready", message: "Ready to process documents", progress: 0 }));
        } catch (error) {
            console.error('Failed to setup session:', error);
            setUploadState(prev => ({
                ...prev,
                status: "error",
                error: "Failed to initialize the prover. Please refresh the page.",
                progress: 0
            }));
        } finally {
            setIsLoading(false);
        }
    };

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();

        const file = inputRef.current?.files?.[0];
        if (!file) {
            setUploadState(prev => ({
                ...prev,
                status: "error",
                error: "Please select a file to upload",
                progress: 0
            }));
            return;
        }

        const formData = new FormData();
        formData.append("file", file);
        formData.append("country", "Portugal");

        try {
            // Step 1: Broadcasting blob transaction
            setUploadState(prev => ({
                ...prev,
                status: "broadcasting",
                message: "Broadcasting data to Hylé network...",
                progress: 20
            }));

            const blobTxHash = await broadcastBlobTx(network, {
                identity: "",
                blobs: [
                    {
                        contractName: "sp1_residency",
                        data: Array.from(new TextEncoder().encode("Portugal")),
                    },
                ]
            });

            // Step 2: Generating proof
            setUploadState(prev => ({
                ...prev,
                status: "proving",
                message: "Generating proof...",
                progress: 40
            }));

            formData.append("tx_hash", blobTxHash);
            const response = await fetch(`http://localhost:${proverPort}/upload`, {
                method: "POST",
                body: formData,
            });

            if (!response.ok) {
                throw new Error("Failed to generate proof");
            }

            const proofBuffer = await response.arrayBuffer();
            const proofBytes = new Uint8Array(proofBuffer);

            // Step 3: Broadcasting proof
            setUploadState(prev => ({
                ...prev,
                status: "broadcasting_proof",
                message: "Broadcasting proof to Hylé network...",
                progress: 70
            }));

            if (!isLoading) {
                await broadcastProofTx(
                    network,
                    blobTxHash,
                    "sp1_residency",
                    proofBytes
                );

                // Step 4: Success
                setUploadState({
                    status: "success",
                    message: "Proof successfully verified and recorded on Hylé!",
                    error: "",
                    progress: 100
                });
            }
        } catch (error) {
            console.error("Process error:", error);
            setUploadState({
                status: "error",
                error: error instanceof Error ? error.message : "Failed to process the document. Please try again.",
                message: "",
                progress: 0
            });
        }
    };

    const getStatusColor = () => {
        switch (uploadState.status) {
            case "error":
                return "text-red-500";
            case "success":
                return "text-green-500";
            default:
                return "text-gray-600";
        }
    };

    return (
        <div className="flex flex-col gap-8 text-center sm:text-center w-full">
            <div className="space-y-6">
                <h3 className="text-xl font-semibold text-gray-800">
                    Upload a Portuguese utility bill to verify residence
                </h3>
                <p className="text-sm text-gray-600">
                    The document must contain a valid ATCUD code and postal code
                </p>
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

                    {uploadState.status !== 'idle' && (
                        <div className="mt-6 space-y-4">
                            <div className="flex items-center justify-center gap-2">
                                {uploadState.status !== 'error' && uploadState.status !== 'success' && uploadState.status !== 'ready' && (
                                    <ProgressCircleRoot size="xs" value={null} colorPalette="gray">
                                        <ProgressCircleRing />
                                    </ProgressCircleRoot>
                                )}
                                <span className={`text-sm font-medium ${getStatusColor()}`}>
                                    {uploadState.message || uploadState.error}
                                </span>
                            </div>
                        </div>
                    )}

                    <button
                        type="submit"
                        disabled={uploadState.status === 'broadcasting' ||
                            uploadState.status === 'proving' ||
                            uploadState.status === 'broadcasting_proof' ||
                            uploadState.status === 'initializing'}
                        className="mt-4 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 
                                transition-colors disabled:bg-gray-400 disabled:cursor-not-allowed"
                    >
                        {uploadState.status === 'idle' || uploadState.status === 'error' || uploadState.status === 'ready' ?
                            'Submit' : 'Processing...'}
                    </button>
                </FileUploadRoot>
            </form>
        </div>
    );
}