import { createAsyncStoragePersister } from "@tanstack/query-async-storage-persister";
import { QueryClient } from "@tanstack/react-query";
import { PersistQueryClientProvider } from "@tanstack/react-query-persist-client";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { useEffect } from "react";
import { DebugPanel } from "~/components/custom/DebugPanel";
// import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { socket } from "~/lib/socket";

const queryClient = new QueryClient();
const asyncStoragePersister = createAsyncStoragePersister({
  storage: window.localStorage,
});

export const Route = createRootRoute({
  component: Root,
});

function Root() {
  useEffect(() => {
    socket.on("connect", () => {
      console.log("connected to ws");
    });
  }, []);

  return (
    <>
      <PersistQueryClientProvider
        client={queryClient}
        persistOptions={{
          persister: asyncStoragePersister,
          dehydrateOptions: {
            shouldDehydrateQuery: (query) => Boolean(query.meta?.persist),
          },
        }}
      >
        <Outlet />
        <DebugPanel />
      </PersistQueryClientProvider>
      {/* <TanStackRouterDevtools /> */}
    </>
  );
}
