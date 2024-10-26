import type { AssetFile } from "./assets.types";
import type { EventPlatform, EventType } from "./events.types";

export type TransformXAlign = "left" | "center" | "right";
export type TransformYAlign = "top" | "center" | "bottom";

export interface WidgetTransform {
  xAlign: TransformXAlign;
  yAlign: TransformYAlign;
  // offset: Vec2Like; // TODO: add offset
  // scale: number; // TODO: add scale
}

interface SpineFrameParams {
  name: string;
  duration: number;
  delay?: number;
}
interface SoundParams {
  file: AssetFile;
  duration: number;
  volume?: number;
  delay?: number;
}
interface SpineAnimationFrame {
  spine: SpineFrameParams;
  sound?: SoundParams;
}

export interface SpineAnimationParams {
  jsonFile: AssetFile;
  atlasFile: AssetFile;
  intro: SpineAnimationFrame;
  idle: SpineAnimationFrame;
  outro: SpineAnimationFrame;
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
  platforms: EventPlatform[];
  transform: WidgetTransform;
  animations: SpineAnimationParams;
  messageBox: WidgetMessageBox;
  titleTemplate: TextTemplate;
  subtitleTemplate: TextTemplate;
}
