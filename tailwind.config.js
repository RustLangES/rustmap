/** @type {import('tailwindcss').Config} */
module.exports = {
  plugins: [
    require('@tailwindcss/typography'),
  ],
  theme: {
    fontFamily: {
      "alfa-slab": ["Alfa Slab One", "sans-serif"],
      "fira-sans": ["Fira Sans", "sans-serif"],
      "work-sans": ["Work Sans", "sans-serif"],
    },
    extend: {
      fontSize: {
        'half': '3.5rem',
      },
      lineHeight: {
        '3lh': '2.8rem',
      },
      backgroundImage: (theme) => ({
        "anuncios": "url('https://i.imgur.com/tDlT9sr.jpg')",
        "kaku-dev": "url('https://www.rustlang-es.org/kaku.avif')",
        "kaku": "url('https://www.rustlang-es.org/kaku.avif')",
      }),
    },
  },
};
