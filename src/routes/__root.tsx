import { Outlet, createRootRoute } from "@tanstack/react-router";
// import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { NavMenu } from "~/components/custom/NavMenu";

export const Route = createRootRoute({
  component: Root,
});

function Root() {
  return (
    <>
      <div className="flex min-h-screen flex-col">
        <NavMenu />
        <div className="flex-1">
          <Outlet />
        </div>
      </div>
      {/* <TanStackRouterDevtools /> */}
    </>
  );
}
