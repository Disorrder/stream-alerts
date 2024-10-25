import type { AssetFile } from "./assets.types";
import type { EventPlatform, EventType } from "./event.types";

export type TransformXAlign = "left" | "center" | "right";
export type TransformYAlign = "top" | "center" | "bottom";

export interface WidgetTransform {
  xAlign: TransformXAlign;
  yAlign: TransformYAlign;
  // offset: Vec2Like; // TODO: add offset
  // scale: number; // TODO: add scale
}

interface SpineAnimation {
  name: string;
  duration: number;
  delay?: number;
}

export interface SpineAnimations {
  jsonFile: AssetFile;
  atlasFile: AssetFile;
  intro: SpineAnimation;
  idle: SpineAnimation;
  outro: SpineAnimation;
}

export interface WidgetMessageBox {
  width: number;
  maxHeight: number;
}

export interface TextTemplate {
  template: string;
  variables: string[];
}

export interface SpineWidget {
  eventType: EventType;
  platform: EventPlatform;
  transform: WidgetTransform;
  animatioNikins: SpineAnimations;
  messageBox: WidgetMessageBox;
  titleTemplate: TextTemplate;
  subtitleTemplate: TextTemplate;
}
