// src/hooks/useThemeSwitcher.ts

import { useEffect, useState } from "react";

// We want to pass the themes array to our switcher, so we tell it that themes is a string[]
const useThemeSwitcher = <T extends readonly string[]>(themes: T) => {
    const [currentTheme, setCurrentTheme] = useState<string>(() => {
        const savedTheme: string = localStorage.getItem("theme") ?? ""
        return themes.includes(savedTheme) ? savedTheme : "autumn";
    });

    // When updating the theme, modify the document to have the theme, 
    // but also make sure that we save it to local storage
    useEffect(() => {
        document.body.setAttribute("data-theme", currentTheme);
        localStorage.setItem("theme", currentTheme);
    }, [currentTheme]);

    

    // Here is where we set the theme, this has validation to ensure that it's a real theme
    // Set theme to type T[number] because that ensures that we are always matching the type of the themes array
    const changeTheme = (theme: T[number]) => {
        themes.includes(theme) ? setCurrentTheme(theme) : console.error("Invalid theme selected");
    };

    return {
        currentTheme,
        changeTheme,
    };
    
}

export default useThemeSwitcher;