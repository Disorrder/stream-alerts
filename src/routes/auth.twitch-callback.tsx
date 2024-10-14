import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";
import { socket } from "~/lib/socket";

export const Route = createFileRoute("/auth/twitch-callback")({
  component: TwitchCallback,
});

function TwitchCallback() {
  const hashParams = new URLSearchParams(location.hash.replace("#", ""));
  const access_token = hashParams.get("access_token");

  useEffect(() => {
    if (!access_token) return;
    socket.emit("set_twitch_token", { access_token });
  }, [access_token]);

  return (
    <div className="px-4 py-2">
      Hello from Twitch Callback!
      <br />
      hash: {access_token}
      <br />
      You can close this page now.
    </div>
  );
}
