
export const Center: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    return (
        <div className="relative h-full bg-inherit">
            <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                {children}
            </div>
        </div>
    )
}