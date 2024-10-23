import { createFileRoute, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/_app/")({
  component: Index,
  loader: () => {
    throw redirect({ to: "/settings" });
  },
});

function Index() {
  return (
    <div className="px-4 py-2">
      <h3>Welcome Home!</h3>
    </div>
  );
}
