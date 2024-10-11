import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/settings")({
  component: Settings,
});

function Settings() {
  return <div className="px-4 py-2">Hello from Settings!</div>;
}
