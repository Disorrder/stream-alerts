import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import TwitchIcon from "~/assets/icons/twitch.svg?react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "~/components/ui/accordion";
import { Avatar } from "~/components/ui/avatar";
import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import { useTwitchUserQuery } from "~/hooks/api-queries";

export const Route = createFileRoute("/_app/settings")({
  component: Settings,
});

function Settings() {
  const twitchUserQuery = useTwitchUserQuery();

  function renderTwitchButton() {
    const { data } = twitchUserQuery;
    return (
      <Button
        variant="twitch"
        disabled={!!data}
        onClick={() => invoke("twitch_open_oauth")}
      >
        <TwitchIcon className="size-5" />
        {!data ? "Add Twitch" : "Connected"}
      </Button>
    );
  }

  return (
    <div className="space-y-2 px-4 py-2">
      <div>Accounts</div>
      <div className="flex gap-2">{renderTwitchButton()}</div>
      <Accordion type="single" collapsible>
        <SettingsTwitch />
      </Accordion>
    </div>
  );
}

function SettingsTwitch() {
  const { data, isLoading } = useTwitchUserQuery();

  return (
    <AccordionItem value="twitch">
      <AccordionTrigger>
        <div className="flex items-center gap-2">
          <TwitchIcon className="size-5 text-twitch/80" /> {data?.display_name}
        </div>
      </AccordionTrigger>
      <AccordionContent className="grid grid-cols-2 gap-3">
        <div>More settings coming soon...</div>
        <Card className="flex flex-col gap-3 p-3">
          <div className="flex items-center gap-3">
            <Avatar src={data?.profile_image_url} className="size-10" />
            <div className="flex-1">
              <div>Followers: 123</div>
              <div>Subscribers: 123</div>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="destructive">Disconnect</Button>
          </div>
        </Card>
      </AccordionContent>
    </AccordionItem>
  );
}
