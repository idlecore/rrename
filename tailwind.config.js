/** @type {import('tailwindcss').Config} **/

export default {
    content: [
        "./src/app.html",
        "./src/**/*.{svelte,js,ts}"
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('tailwind-scrollbar'),
    ],
};
