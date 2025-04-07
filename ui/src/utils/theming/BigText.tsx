export const BigText: React.FC<BigTextProps> = (props: BigTextProps) => {
    return (
        <span className="text-white text-lg font-mono">{props.text}</span>
    )
}

interface BigTextProps {
    text: string
}