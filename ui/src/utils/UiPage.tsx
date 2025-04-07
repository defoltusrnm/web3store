import React from 'react';

export const UiPage: React.FC<UiPageProps> = ({ children: inner }) => {
    return (
        <div className='h-screen bg-background'>
            {inner}
        </div>
    )
}

interface UiPageProps {
    children: React.ReactNode;
}