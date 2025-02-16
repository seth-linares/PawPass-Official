// src/components/ThemeSwitcher.tsx

import React from "react";
import useThemeSwitcher from "@/hooks/settings/useThemeSwitcher";
import { ComboBox } from "../ComboBox";
import { capitalizeFirstLetter } from "@/hooks/useComboBox";


const ThemeSwitcher: React.FC = () => {

    const themes = [
        "acid", "aqua", "autumn", "black", "bumblebee", "business", "cmyk",
        "coffee", "corporate", "cupcake", "cyberpunk", "dark", "dim", "dracula",
        "emerald", "fantasy", "forest", "garden", "halloween", "lemonade",
        "light", "lofi", "luxury", "night", "nord", "pastel", "retro", "sunset",
        "synthwave", "valentine", "winter", "wireframe"
    ] as const;

    const {
        currentTheme,
        changeTheme,
    } = useThemeSwitcher(themes);


    return (
        <div className="card bg-base-200 shadow-xl transition-all duration-300 hover:shadow-2xl">
            <div className="card-body items-center text-center">
                <h2 className="card-title font-bold mb-4">
                    Theme: {capitalizeFirstLetter(currentTheme)}
                </h2>
                
                <div className="form-control w-full max-w-xs">
                    <ComboBox
                        items={themes}
                        defaultValue={currentTheme}
                        onValueChange={(value) => changeTheme(value as typeof themes[number])}
                        label="Theme"
                        showLabel={true}
                        description="Select your preferred theme"
                        placeholder="Select theme"
                        searchPlaceholder="Search themes..."
                        emptyMessage="No theme found"
                        capitalizeItems={true}
                    />
                </div>
            </div>
        </div>
    );

}

export default ThemeSwitcher;