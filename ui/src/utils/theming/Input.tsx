
export const Input: React.FC<{
    placeholder: string;
    value: string;
    onChange: (e: string) => void;
}> = ({ placeholder, value, onChange }) => {
    return (
        <input
            type='text'
            placeholder={placeholder}
            value={value}
            onChange={e => onChange(e.target.value)}
            className="font-mono px-4 text-white bg-background py-2 border rounded-lg border-secondary focus:outline-none focus:ring-2 focus:ring-primary"
        />
    );
};