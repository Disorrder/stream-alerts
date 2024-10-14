import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_app/moderation')({
  component: Moderation,
})

function Moderation() {
  return <div className="px-4 py-2">Hello from Moderation!</div>
}
