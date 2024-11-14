import {
  FileUploadDropzone,
  FileUploadList,
  FileUploadRoot,
} from "@/components/ui/file-button"

export default function Home() {
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <header className="row-start-1 flex items-center justify-center gap-4">
        <h1 className="text-4xl font-bold">Proof of Residency</h1>
      </header>
      
      <main className="flex flex-col gap-12 row-start-2 items-center sm:items-start w-full max-w-4xl">
        <div className="flex flex-col gap-8 text-center sm:text-center w-full">
          <div className="space-y-6">
            <h3 className="text-xl font-semibold text-gray-800">
              Upload a Portuguese utility bill to verify residence
            </h3>
          </div>

          <div className="w-full">
            <FileUploadRoot alignItems="stretch" maxFiles={10}>
              <FileUploadDropzone
                label="Drag and drop here to upload"
                description=".pdf files up to 5MB"
              />
              <FileUploadList />
            </FileUploadRoot>
          </div>
        </div>
      </main>

      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">
      </footer>
    </div>
  );
}