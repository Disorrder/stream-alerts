import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  return (
    <div className="px-4 py-2">
      <h3>Welcome Home!</h3>
    </div>
  );
}
