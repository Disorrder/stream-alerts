import { LogicalPosition, LogicalSize, Window } from "@tauri-apps/api/window";

export async function initWindowSize() {
  const w = 1000;
  const h = window.screen.height;
  await Window.getCurrent().setSize(new LogicalSize(w, h));

  // set position to right side of screen
  const left = window.screen.width - w;
  await Window.getCurrent().setPosition(new LogicalPosition(left, 0));
}
