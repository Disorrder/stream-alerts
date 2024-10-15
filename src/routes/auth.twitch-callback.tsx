import { createFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { socket } from "~/lib/socket";

export const Route = createFileRoute("/auth/twitch-callback")({
  component: TwitchCallback,
});

function TwitchCallback() {
  const hashParams = new URLSearchParams(location.hash.replace("#", ""));
  const access_token = hashParams.get("access_token");
  const [done, setDone] = useState(false);

  useEffect(() => {
    if (!access_token) return;
    socket.emit("twitch:set_token", { access_token });
    socket.on("twitch:token_set", (token: string) => {
      if (token !== access_token) {
        console.error("[JS] Token mismatch:", token, access_token);
        return;
      }
      setDone(true);
    });
  }, [access_token]);

  return (
    <div className="px-4 py-2">
      Hello from Twitch Callback!
      <br />
      hash: {access_token}
      <br />
      {done ? "You can close this page now." : "Loading..."}
    </div>
  );
}
