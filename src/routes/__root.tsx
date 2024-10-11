import { Link, Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { useEffect } from "react";
import { setDefaultWindowSize } from "~/lib/window";

export const Route = createRootRoute({
  component: Root,
});

function Root() {
  useEffect(() => {
    setDefaultWindowSize();
  }, []);

  return (
    <>
      <div className="flex gap-2 p-2">
        <Link to="/" className="[&.active]:font-bold">
          Home
        </Link>{" "}
        <Link to="/settings" className="[&.active]:font-bold">
          Settings
        </Link>
      </div>
      <hr />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
}
