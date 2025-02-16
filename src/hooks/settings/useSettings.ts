import { useState, useEffect } from 'react';

export type LogoPreference = 'classic' | 'silly';

export const useSettings = () => {
    const [isPreviewOpen, setIsPreviewOpen] = useState(false);
    const [logoPreference, setLogoPreference] = useState<LogoPreference>(() => {
        const saved = localStorage.getItem('logoPreference');
        return (saved as LogoPreference) || 'classic';
    });

    useEffect(() => {
        localStorage.setItem('logoPreference', logoPreference);
    }, [logoPreference]);

    const togglePreview = () => setIsPreviewOpen(!isPreviewOpen);

    const toggleLogo = () => {
        setLogoPreference(prev => prev === 'classic' ? 'silly' : 'classic');
    };

    return {
        isPreviewOpen,
        togglePreview,
        logoPreference,
        toggleLogo
    };
};
