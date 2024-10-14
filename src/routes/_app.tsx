import { Outlet, createFileRoute } from "@tanstack/react-router";
import { NavMenu } from "~/components/custom/NavMenu";

export const Route = createFileRoute("/_app")({
  component: AppLayout,
});

function AppLayout() {
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
