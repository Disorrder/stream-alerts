import { RouterProvider, createRouter } from "@tanstack/react-router";
import { useEffect } from "react";
import "./index.css";
import { initWindowSize } from "./lib/window";
import { routeTree } from "./routeTree.gen";

const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

function App() {
  useEffect(() => {
    initWindowSize();
  }, []);

  return <RouterProvider router={router} />;
}

export default App;
