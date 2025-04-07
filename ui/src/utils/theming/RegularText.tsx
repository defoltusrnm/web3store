import React from "react";


export const RegularText: React.FC<RegularTextProps> = (props: RegularTextProps) => {
    return (
        <span className="text-white text-base font-mono">{props.text}</span>
    )
}

interface RegularTextProps {
    text: string
}