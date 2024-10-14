import { Outlet, createRootRoute } from "@tanstack/react-router";
import { useEffect } from "react";
// import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { socket } from "~/lib/socket";

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
      <Outlet />
      {/* <TanStackRouterDevtools /> */}
    </>
  );
}
