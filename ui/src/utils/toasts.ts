import { toast } from "react-toastify";

export const notifyError = (args: NotifyErrorArgs): void => {
    toast.error(args.caption, {
        position: "top-left",
        className: 'bg-foreground text-white'
    });
}

export type NotifyErrorArgs = {
    caption: string
}