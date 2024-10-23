import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_app/widgets")({
  component: Widgets,
});

function Widgets() {
  return (
    <div className="px-4 py-2">
      <h3>Coming Soon!</h3>
    </div>
  );
}
