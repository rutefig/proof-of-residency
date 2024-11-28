import FileUpload from "./components/FileUpload";

export default function Home() {
  

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <header className="row-start-1 flex items-center justify-center gap-4">
        <h1 className="text-4xl font-bold">Proof of Residency</h1>
      </header>
      
      <main className="flex flex-col gap-12 row-start-2 items-center sm:items-start w-full max-w-4xl">
        <FileUpload />
      </main>

      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">
      </footer>
    </div>
  );
}