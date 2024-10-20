import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_app/")({
  component: Index,
});

function Index() {
  return (
    <div className="px-4 py-2">
      <h3>Welcome Home!</h3>
      <br />
      <h1>LOCATION: {window.location.href}</h1>
    </div>
  );
}
