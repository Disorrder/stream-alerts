import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/w")({
  component: W,
});

export function W() {
  return <div>W</div>;
}
