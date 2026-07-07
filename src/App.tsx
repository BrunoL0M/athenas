import { useEffect, useState } from "react";
import { getAppInfo, type AppInfo } from "./lib/ipc";

function App() {
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);

  useEffect(() => {
    getAppInfo().then(setAppInfo);
  }, []);

  return (
    <main className="min-h-screen flex items-center justify-center">
      <div className="card p-8 max-w-sm w-full">
        <h1 className="font-serif text-2xl mb-4">
          {appInfo ? appInfo.name : "Cargando..."}
        </h1>
        {appInfo && (
          <>
            <p className="text-text-muted text-sm mb-2">v{appInfo.version}</p>
            <div className="flex gap-2 flex-wrap">
              {appInfo.stack.map((tech) => (
                <span
                  key={tech}
                  className="text-xs px-2 py-1 rounded-full border border-border text-text-muted"
                >
                  {tech}
                </span>
              ))}
            </div>
          </>
        )}
      </div>
    </main>
  );
}

export default App;
