import FileUpload from "./components/FileUpload";

export default function Home() {
  return (
    <div className="h-screen flex flex-col">
      <header className="flex-none py-8">
        <h1 className="text-4xl font-bold text-center">Proof of Residency</h1>
      </header>
      
      <main className="flex-1 flex flex-col items-center px-4 sm:px-8">
        <div className="w-full max-w-4xl">
          <FileUpload />
        </div>
      </main>

      <footer className="flex-none py-6 flex gap-6 flex-wrap items-center justify-center">
      </footer>
    </div>
  );
}