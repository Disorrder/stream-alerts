import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import TwitchIcon from "~/assets/icons/twitch.svg?react";
import { Button } from "~/components/ui/button";
import { useSettingsStore } from "~/store/settings.store";

export const Route = createFileRoute("/_app/settings")({
  component: Settings,
});

function Settings() {
  const twitch = useSettingsStore((state) => state.twitch);

  async function handleAuthenticateTwitch() {
    await invoke("twitch_open_oauth");
  }

  return (
    <div className="space-y-2 px-4 py-2">
      <div>Accounts</div>
      <div className="flex gap-2">
        <Button variant="twitch" onClick={handleAuthenticateTwitch}>
          <TwitchIcon className="size-5" />
          Add Twitch
        </Button>
      </div>
      <div>
        <div>Twitch: {twitch?.display_name}</div>
      </div>
    </div>
  );
}
