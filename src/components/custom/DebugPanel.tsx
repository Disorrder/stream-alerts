import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { Button } from "~/components/ui/button";
import { socket } from "~/lib/socket";
import { cn } from "~/lib/utils";

export function DebugPanel() {
  const [open, setOpen] = useState(true);
  const [messages, setMessages] = useState<string[]>([
    `Location: ${window.location.href}`,
  ]);

  useEffect(() => {
    socket.on("debug", (_message: string) => {
      const message = `${new Date().toLocaleTimeString()} [WS] ${_message}`;
      setMessages((prev) => [...prev, message]);
    });
    listen("debug", (event) => {
      const message = `${new Date().toLocaleTimeString()} [TAURI] ${
        event.payload as string
      }`;
      setMessages((prev) => [...prev, message]);
    });
  }, []);

  return (
    <div
      className={cn(
        "absolute right-0 bottom-0 left-0 border-t",
        open ? "max-h-64" : "max-h-0",
      )}
    >
      <div className="absolute right-0 bottom-full m-2">
        <Button variant="ghost" size="sm" onClick={() => setOpen(!open)}>
          {open ? "Hide debug" : "Show debug"}
        </Button>
      </div>
      <div className="h-full overflow-auto p-2 text-sm">
        {messages.map((message) => (
          <div key={message}>{message}</div>
        ))}
      </div>
    </div>
  );
}
