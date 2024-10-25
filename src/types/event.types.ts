type PlatformType = "stream" | "donation";
export type StreamPlatform = "twitch";
export type DonationPlatform = "donation-alerts";
export type EventPlatform = StreamPlatform | DonationPlatform;

export const EVENT_PLATFORMS_MAP: Record<string, EventPlatform[]> = {
  follow: ["twitch"],
  subscription: ["twitch"],
  sub_gift: ["twitch"],
  raid: ["twitch"],
  // resub: ["twitch"], //? Discuss should I separate sub and resub
  /** Custom event for multiple follows */
  mass_follow: ["twitch"], // TODO
  /** Custom event for multiple gift subscriptions */
  sub_bomb: ["twitch"], // TODO
  /* Donation */
  donation: ["donation-alerts"],
} as const;

export type EventType = keyof typeof EVENT_PLATFORMS_MAP;

export type TwitchSubscriptionTier = "tier 1" | "tier 2" | "tier 3" | "prime";

export interface BaseEvent {
  type: EventType;
  platform: EventPlatform;
  createdAt: Date;
  playedAt?: Date;
}

export interface StreamFollowEvent extends BaseEvent {
  type: "follow";
  platform: (typeof EVENT_PLATFORMS_MAP)["follow"][number];
  userName: string;
}

export interface StreamSubscribeEvent extends BaseEvent {
  type: "subscription";
  platform: (typeof EVENT_PLATFORMS_MAP)["subscription"][number];
  userName: string;
  tier: TwitchSubscriptionTier;
  /* Resub */
  months?: number;
  monthsStreak?: number | null;
  message?: string;
}

export interface StreamGiftEvent extends BaseEvent {
  type: "sub_gift";
  platform: (typeof EVENT_PLATFORMS_MAP)["sub_gift"][number];
  gifterName: string | null;
  userName: string;
  tier: TwitchSubscriptionTier;
  months: number;
  cumulativeGifts: number | null;
}

export interface StreamRaidEvent extends BaseEvent {
  type: "raid";
  platform: (typeof EVENT_PLATFORMS_MAP)["raid"][number];
  raiderName: string;
  viewerCount: number;
}

/* Donation */

export interface DonationEvent extends BaseEvent {
  type: "donation";
  platform: (typeof EVENT_PLATFORMS_MAP)["donation"][number];
  userName: string;
  amount: number;
  currency: string;
  convertedAmount: number;
  message: string;
}
