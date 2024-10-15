import { createFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { socket } from "~/lib/socket";

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

  useEffect(() => {
    if (!code) return;
    socket.emit("twitch:auth_by_code", { code });
    socket.on("twitch:auth_by_code:response", (res: string) => {
      if (res !== "ok") {
        console.error("[JS] Something went wrong:", res);
        return;
      }
      setDone(true);
    });
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
