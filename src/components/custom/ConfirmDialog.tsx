import type { DialogProps as PrimitiveDialogProps } from "@radix-ui/react-dialog";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { Button } from "../ui/button";

// type DialogProps = Omit<PrimitiveDialogProps, "onOpenChange">;

interface BaseDialogProps extends PrimitiveDialogProps {
  title: string;
  description: string;
}

interface PropsWithOnConfirm extends BaseDialogProps {
  /** Provide the confirm button handler, don't use with `confirmButton` */
  onConfirm: () => void;
}

interface PropsWithConfirmButton extends BaseDialogProps {
  /** Override the default confirm button, don't use with `onConfirm` */
  confirmButton: React.ReactNode;
}

type Props = PropsWithOnConfirm | PropsWithConfirmButton;

export function ConfirmDialog(props: Props) {
  const { title, description, children, ...rest } = props;

  function renderConfirmButton() {
    if ("confirmButton" in props) return props.confirmButton;
    return <Button onClick={props.onConfirm}>Confirm</Button>;
  }

  return (
    <Dialog {...rest}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        {children}
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <DialogClose asChild>{renderConfirmButton()}</DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
