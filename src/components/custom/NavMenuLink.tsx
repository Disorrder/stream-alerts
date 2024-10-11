import { Link, useMatchRoute } from "@tanstack/react-router";
import { cn } from "~/lib/utils";
import { Button } from "../ui/button";

interface Props extends React.ComponentProps<typeof Link> {
  icon?: boolean;
}

export function NavMenuLink(props: Props) {
  const { icon, className, children, ...linkProps } = props;
  const matchRoute = useMatchRoute();
  const isActive = matchRoute({ to: props.to as string });
  const btnVariant = isActive ? "secondary" : "ghost";

  return (
    <Link className={cn("group", className)} {...linkProps}>
      <Button
        variant={btnVariant}
        size={icon ? "icon" : undefined}
        className="gap-2"
      >
        {children as React.ReactNode}
      </Button>
    </Link>
  );
}
