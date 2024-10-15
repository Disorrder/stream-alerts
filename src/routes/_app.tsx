import { Outlet, createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";
import { NavMenu } from "~/components/custom/NavMenu";
import { socket } from "~/lib/socket";

export const Route = createFileRoute("/_app")({
  component: AppLayout,
});

function AppLayout() {
  useEffect(() => {
    socket.on("profile", (profile) => {
      console.log("[JS] Profile:", profile);
    });
  }, []);

  return (
    <>
      <div className="flex min-h-screen flex-col">
        <NavMenu />
        <div className="flex-1">
          <Outlet />
        </div>
      </div>
    </>
  );
}
