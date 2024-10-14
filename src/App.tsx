import { RouterProvider, createRouter } from "@tanstack/react-router";
import { useEffect } from "react";
import "./index.css";
import { initWindowSize } from "./lib/window";
import { routeTree } from "./routeTree.gen";
import { register } from "@tauri-apps/plugin-deep-link";

const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

function App() {
  useEffect(() => {
    register("alerts");
    initWindowSize();

    return () => {
      // unregister("alerts");
    };
  }, []);

  return <RouterProvider router={router} />;
}

export default App;
