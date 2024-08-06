/** @type {import('tailwindcss').Config} */
module.exports = {
  corePlugins: {
      backgroundClip: true, // Ensure this is enabled if necessary
  },
  content: [
    "./src/*.rs",
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      fontFamily: {
        'manrope': ['Manrope', 'sans-serif'],
      },
      colors: {
        'vista': '#FCF8F5',
        'vista-dark': '#F3EFEE',
        'jungle-green': '#171717',
        'grey-cloud': '#BAB5B2',
        'black': '#000000',
        // Gradients
        'grad-1': '#EEC5BD',
        'grad-2': '#F3B1A5',
        'grad-3': '#C6CAF6',
        'grad-4': '#DAD9E9',
      }
    },
  },
  plugins: [],
}

