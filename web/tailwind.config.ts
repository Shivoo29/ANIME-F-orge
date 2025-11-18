import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "#FF6B35",
        secondary: "#004E89",
        accent: "#F7B801",
        dark: "#000000",
        light: "#FFFFFF",
      },
      borderWidth: {
        brutal: "4px",
      },
      boxShadow: {
        brutal: "8px 8px 0px 0px #000000",
        "brutal-sm": "4px 4px 0px 0px #000000",
        "brutal-lg": "12px 12px 0px 0px #000000",
      },
      fontFamily: {
        sans: ["var(--font-geist-sans)", "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [],
};

export default config;
