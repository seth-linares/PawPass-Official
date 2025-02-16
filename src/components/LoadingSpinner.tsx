// src/components/LoadingSpinner.tsx

import { LoadingSpinnerProps } from '@/types/types';
import React from 'react';

const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({ 
    size = 'md',
    label,
    className = '',
    inline = false
}) => {
    if (inline) {
        return <span className={`loading loading-spinner loading-${size}`} />;
    }

    return (
        <div className={`flex flex-col items-center justify-center ${className}`}>
            <span className={`loading loading-spinner loading-${size}`}></span>
            {label && (
                <span className="mt-2 text-sm text-base-content/80">{label}</span>
            )}
        </div>
    );
};

export default LoadingSpinner;