import { CogIcon, FileClockIcon, InboxIcon, SpeechIcon } from "lucide-react";
import { NavMenuLink } from "./NavMenuLink";

export function NavMenu() {
  return (
    <div className="flex gap-2 bg-slate-950 px-4 py-2">
      {/* <NavMenuLink to="/" icon>
        <HomeIcon className="size-5" />
      </NavMenuLink> */}
      <NavMenuLink to="/events">
        <InboxIcon className="size-5" />
        Events
      </NavMenuLink>
      <NavMenuLink to="/logs">
        <FileClockIcon className="size-5" />
        Logs
      </NavMenuLink>
      <NavMenuLink to="/widgets">
        <SpeechIcon className="size-5" />
        Widgets
      </NavMenuLink>
      <div className="flex-1" />
      <NavMenuLink to="/settings" icon>
        <CogIcon className="size-5" />
      </NavMenuLink>
    </div>
  );
}
