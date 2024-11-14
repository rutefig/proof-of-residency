import {
  FileUploadDropzone,
  FileUploadList,
  FileUploadRoot,
} from "@/components/ui/file-button"

export default function Home() {
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <header className="row-start-1 flex items-center justify-center gap-4">
        <h1 className="text-4xl font-bold">Proof of Residency</h1>
      </header>
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <div>
          <h3>Upload a Portuguese utility bill to verify residence</h3>
          <FileUploadRoot maxW="xl" alignItems="stretch" maxFiles={1}>
            <FileUploadDropzone
              label="Drag and drop here to upload"
              description=".png, .jpg up to 5MB"
            />
            <FileUploadList />
          </FileUploadRoot>
        </div>
      </main>
      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">

      </footer>
    </div>
  );
}
