import React from "react";

export const Button: React.FC<ButtonProps> = (props: ButtonProps) => {
    return (
        <button
            onClick={props.handler}
            className="px-4 py-2 rounded bg-background text-white hover:bg-primary/80 transition duration-200"
        >
            {props.children}
        </button>
    )
}

export interface ButtonProps {
    children: React.ReactNode
    handler?: () => void
}