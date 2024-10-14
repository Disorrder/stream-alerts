/// <reference types="vite/client" />

declare module "*.svg?react" {
  import type { FunctionComponent, SVGAttributes } from "react";
  const content: FunctionComponent<SVGAttributes<SVGElement>>;
  export default content;
}
