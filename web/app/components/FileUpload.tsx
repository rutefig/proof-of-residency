"use client";

import { useState, useRef } from "react";
import {
    FileUploadDropzone,
    FileUploadList,
    FileUploadRoot,
} from "@/components/ui/file-button";

export default function FileUpload() {
    const [uploadStatus, setUploadStatus] = useState<string>("");
    const inputRef = useRef<HTMLInputElement>(null);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        
        const file = inputRef.current?.files?.[0];
        if (!file) {
            setUploadStatus("Error: No file selected");
            return;
        }

        const formData = new FormData();
        formData.append("file", file);

        try {
            setUploadStatus("Uploading...");

            const response = await fetch("http://localhost:8080/upload", {
                method: "POST",
                body: formData,
            });

            const data = await response.json();
            console.log("Server response:", data);

            if (response.ok) {
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
                <div className={`text-sm font-medium ${
                    uploadStatus.includes("Error") ? "text-red-500" : "text-green-500"
                }`}>
                    {uploadStatus}
                </div>
            )}
        </div>
    );
}