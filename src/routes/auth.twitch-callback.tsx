import { useMutation } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { api } from "~/lib/api";

export const Route = createFileRoute("/auth/twitch-callback")({
  component: TwitchCallback,
  validateSearch: (search) => {
    return {
      code: search.code as string,
    };
  },
});

function TwitchCallback() {
  const { code } = Route.useSearch();
  const [done, setDone] = useState(false);

  const sendCodeMutation = useMutation({
    mutationFn: (code: string) => api.post("/twitch/auth/code", { code }),
    onSuccess: () => {
      setDone(true);
    },
  });

  // biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
  useEffect(() => {
    if (!code) return;
    sendCodeMutation.mutate(code);
  }, [code]);

  return (
    <div className="px-4 py-2">
      Hello from Twitch Callback!
      <br />
      Auth code: {code}
      <br />
      {done ? "You can close this page now." : "Loading..."}
    </div>
  );
}
