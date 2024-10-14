import { Outlet, createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth')({
  component: AuthLayout,
})

function AuthLayout() {
  return (
    <div className="px-4 py-2">
      Hello from Auth Layout!
      <Outlet />
    </div>
  )
}
