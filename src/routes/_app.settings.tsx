import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import TwitchIcon from "~/assets/icons/twitch.svg?react";
import { ConfirmDialog } from "~/components/custom/ConfirmDialog";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "~/components/ui/accordion";
import { Avatar } from "~/components/ui/avatar";
import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import { useTwitchDetachQuery, useTwitchUserQuery } from "~/hooks/api-queries";
import { api } from "~/lib/api";

export const Route = createFileRoute("/_app/settings")({
  component: Settings,
});

function Settings() {
  const twitchUserQuery = useTwitchUserQuery();

  function renderTwitchButton() {
    const { data, isLoading } = twitchUserQuery;
    return (
      <Button
        variant="twitch"
        disabled={!!data || isLoading}
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
  const { data } = useTwitchUserQuery();
  const detachMutation = useTwitchDetachQuery();

  const followersQuery = useQuery({
    queryKey: ["twitch", "followers"],
    queryFn: () => api.get<number>("/twitch/followers").then((res) => res.data),
  });

  const [detachDialogOpen, setDetachDialogOpen] = useState(false);
  if (!data) return null;
  return (
    <>
      <AccordionItem value="twitch" disabled={!data}>
        <AccordionTrigger>
          <div className="flex items-center gap-2">
            <TwitchIcon className="size-5 text-twitch/80" />
            {data.display_name}
          </div>
        </AccordionTrigger>
        <AccordionContent className="grid grid-cols-2 gap-3">
          <div>More settings coming soon...</div>
          <Card className="flex flex-col gap-3 p-3">
            <div className="flex items-center gap-3">
              <Avatar src={data.profile_image_url} className="size-10" />
              <div className="flex-1">
                <div>Followers: {followersQuery.data || "Loading..."}</div>
                <div>Subscribers: TODO</div>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <Button
                variant="destructive"
                onClick={() => setDetachDialogOpen(true)}
              >
                Disconnect
              </Button>
            </div>
          </Card>
        </AccordionContent>
      </AccordionItem>
      <ConfirmDialog
        open={detachDialogOpen}
        onOpenChange={setDetachDialogOpen}
        title="Disconnect Twitch"
        description="Are you sure you want to disconnect your Twitch account?"
        confirmButton={
          <Button variant="destructive" onClick={() => detachMutation.mutate()}>
            Disconnect
          </Button>
        }
      />
    </>
  );
}
