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
import { TimelineConnector, TimelineContent, TimelineDescription, TimelineItem, TimelineRoot, TimelineTitle } from "@/components/ui/timeline";
import { LuCheck } from "react-icons/lu";
import { DialogCloseTrigger, DialogContent, DialogRoot } from "@/components/ui/dialog";

enum UploadStatus {
    Idle = "idle",
    Initializing = "initializing",
    Ready = "ready",
    Broadcasting = "broadcasting",
    Proving = "proving",
    Verifying = "verifying",
    Success = "success",
    Error = "error",
}

export default function FileUpload() {
    const [uploadState, setUploadState] = useState({
        status: UploadStatus.Idle,
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
            setUploadState(prev => ({ ...prev, status: UploadStatus.Initializing, message: "Setting up prover session..." }));
            const { session_id, prover_port } = await createProverSession();
            setSessionId(session_id);
            await setProverPort(prover_port);
            setUploadState(prev => ({ ...prev, status: UploadStatus.Initializing, message: "Registering contracts...", progress: 60 }));
            await ensureContractsRegistered(prover_port);
            setUploadState(prev => ({ ...prev, status: UploadStatus.Ready, message: "Ready to process documents", progress: 0 }));
        } catch (error) {
            console.error('Failed to setup session:', error);
            setUploadState(prev => ({
                ...prev,
                status: UploadStatus.Error,
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
                status: UploadStatus.Error,
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
                status: UploadStatus.Broadcasting,
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
                status: UploadStatus.Proving,
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
                status: UploadStatus.Verifying,
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
                    status: UploadStatus.Success,
                    message: "Proof successfully verified and settled on Hylé!",
                    error: "",
                    progress: 100
                });
            }
        } catch (error) {
            console.error("Process error:", error);
            setUploadState({
                status: UploadStatus.Error,
                error: error instanceof Error ? error.message : "Failed to process the document. Please try again.",
                message: "",
                progress: 0
            });
        }
    };

    const getStatusColor = () => {
        switch (uploadState.status) {
            case UploadStatus.Error:
                return "text-red-500";
            case UploadStatus.Success:
                return "text-green-500";
            default:
                return "text-gray-600";
        }
    };

    return (
        <div className="flex flex-col gap-8 text-center sm:text-center w-full">
            <DialogRoot open={uploadState.status === UploadStatus.Success} placement="center">
                <DialogContent className="p-8 max-w-md mx-auto rounded-3xl shadow-xl bg-white">
                    <div className="flex flex-col items-center gap-6 text-center">
                        {/* Top confetti emoji */}
                        <div className="text-4xl">🎊</div>

                        {/* Main content */}
                        <div className="space-y-4">
                            <h3 className="text-3xl font-semibold text-gray-800">
                                Congratulations! 🇵🇹
                            </h3>
                            <p className="text-lg text-gray-600">
                                You&apos;ve successfully verified your Portugal residency!
                                <span className="inline-block ml-2">✅</span>
                            </p>
                        </div>

                        {/* Info box */}
                        <div className="w-full text-gray-600 text-base bg-gray-50 p-4 rounded-xl">
                            <span className="mr-2">💡</span>
                            Your verification proof has been securely stored
                        </div>

                        {/* Close button */}
                        <DialogCloseTrigger
                            onClick={() => setUploadState(prev => ({
                                ...prev,
                                status: UploadStatus.Ready,
                                message: "Ready to process documents",
                                progress: 0
                            }))}
                        >
                        </DialogCloseTrigger>
                    </div>
                </DialogContent>
            </DialogRoot>

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

                    {uploadState.status !== UploadStatus.Idle && (
                        <div className="mt-6 space-y-4">
                            <div className="flex items-center justify-center gap-2 w-fit justify-self-center">
                                {(uploadState.status === UploadStatus.Initializing || uploadState.status === UploadStatus.Ready) &&
                                    (
                                        <>
                                            {uploadState.status !== UploadStatus.Ready && (
                                                <ProgressCircleRoot size="xs" value={null} colorPalette="gray">
                                                    <ProgressCircleRing />
                                                </ProgressCircleRoot>
                                            )}
                                            <span className={`text-sm font-medium ${getStatusColor()}`}>
                                                {uploadState.message || uploadState.error}
                                            </span>
                                        </>
                                    )
                                }
                                {(uploadState.status === UploadStatus.Broadcasting ||
                                    uploadState.status === UploadStatus.Proving ||
                                    uploadState.status === UploadStatus.Verifying ||
                                    uploadState.status === UploadStatus.Success) && (
                                        <TimelineRoot variant="subtle">
                                            <TimelineItem
                                                className={uploadState.status === UploadStatus.Broadcasting ? "animate-pulse" : ""}
                                            >
                                                <TimelineConnector
                                                    className={
                                                        uploadState.status === UploadStatus.Proving ||
                                                            uploadState.status === UploadStatus.Verifying ||
                                                            uploadState.status === UploadStatus.Success ? "bg-green-700" : ""
                                                    }
                                                >
                                                    <LuCheck
                                                        color={uploadState.status === UploadStatus.Proving ||
                                                            uploadState.status === UploadStatus.Verifying ||
                                                            uploadState.status === UploadStatus.Success ? "white" : ""}
                                                    />
                                                </TimelineConnector>
                                                <TimelineContent>
                                                    <TimelineTitle>Broadcasting data</TimelineTitle>
                                                </TimelineContent>
                                            </TimelineItem>

                                            <TimelineItem
                                                className={uploadState.status === UploadStatus.Proving ? "animate-pulse" : ""}
                                            >
                                                <TimelineConnector
                                                    className={
                                                        uploadState.status === UploadStatus.Verifying ||
                                                            uploadState.status === UploadStatus.Success ? "bg-green-700" : ""
                                                    }
                                                >
                                                    <LuCheck
                                                        color={uploadState.status === UploadStatus.Verifying ||
                                                            uploadState.status === UploadStatus.Success ? "white" : ""}
                                                    />
                                                </TimelineConnector>
                                                <TimelineContent>
                                                    <TimelineTitle>Generating proof</TimelineTitle>
                                                    <TimelineDescription>
                                                        {uploadState.status === UploadStatus.Proving ? "This may take a few minutes, please don't close this tab and go get yourself a cup of coffee :)" : ""}
                                                    </TimelineDescription>
                                                </TimelineContent>
                                            </TimelineItem>

                                            <TimelineItem
                                                className={uploadState.status === UploadStatus.Verifying ? "animate-pulse" : ""}
                                            >
                                                <TimelineConnector
                                                    className={
                                                        uploadState.status === UploadStatus.Success ? "bg-green-700" : ""
                                                    }
                                                >
                                                    <LuCheck
                                                        color={uploadState.status === UploadStatus.Success ? "white" : ""}
                                                    />
                                                </TimelineConnector>
                                                <TimelineContent>
                                                    <TimelineTitle>Verifying proof</TimelineTitle>
                                                </TimelineContent>
                                            </TimelineItem>
                                        </TimelineRoot>
                                    )}
                            </div>
                        </div>
                    )}



                    <button
                        type="submit"
                        disabled={uploadState.status === UploadStatus.Broadcasting ||
                            uploadState.status === UploadStatus.Proving ||
                            uploadState.status === UploadStatus.Verifying ||
                            uploadState.status === UploadStatus.Initializing}
                        className="mt-4 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 
                                transition-colors disabled:bg-gray-400 disabled:cursor-not-allowed"
                    >
                        {uploadState.status === UploadStatus.Idle ||
                            uploadState.status === UploadStatus.Error ||
                            uploadState.status === UploadStatus.Ready ||
                            uploadState.status === UploadStatus.Success ?
                            'Submit' : 'Processing...'}
                    </button>
                </FileUploadRoot>
            </form>
        </div>
    );
}