/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,ts}",
  ],
  theme: {
    extend: {
      colors:{
        "dark":"#1f1f1f",
        "light":"#ffffff"
      }
    },
  },
  plugins: [],
}

