/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'raw-walnut': {
          50: '#faf8f5',
          100: '#f2ede4',
          200: '#e4d9c8',
          300: '#d3c0a5',
          400: '#c0a380',
          500: '#b08c64',
          600: '#a37a56',
          700: '#876349',
          800: '#6e5140',
          900: '#5a4436',
          950: '#30231b',
        },
        'washed-linen': {
          50: '#fdfcfa',
          100: '#f9f6f0',
          200: '#f3ede0',
          300: '#ebe0cc',
          400: '#e0cfb3',
          500: '#d4bc99',
          600: '#c4a67d',
          700: '#a98c62',
          800: '#8a7251',
          900: '#715e44',
          950: '#3c3124',
        },
        'iron-hardware': {
          50: '#f6f6f6',
          100: '#e7e7e7',
          200: '#d1d1d1',
          300: '#b0b0b0',
          400: '#888888',
          500: '#6d6d6d',
          600: '#5d5d5d',
          700: '#4f4f4f',
          800: '#454545',
          900: '#3d3d3d',
          950: '#262626',
        },
        'natural-oak': {
          50: '#faf8f3',
          100: '#f3efe3',
          200: '#e6dcc5',
          300: '#d5c59f',
          400: '#c4ab78',
          500: '#b7965c',
          600: '#aa844f',
          700: '#8e6b43',
          800: '#73573b',
          900: '#5e4832',
          950: '#32251a',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        serif: ['Georgia', 'serif'],
      },
      borderRadius: {
        'blob': '60% 40% 30% 70% / 60% 30% 70% 40%',
        'blob-2': '30% 70% 70% 30% / 30% 30% 70% 70%',
        'organic': '40% 60% 50% 50% / 50% 50% 60% 40%',
      },
      animation: {
        'blob': 'blob 8s ease-in-out infinite',
        'blob-slow': 'blob 12s ease-in-out infinite',
        'float': 'float 6s ease-in-out infinite',
      },
      keyframes: {
        blob: {
          '0%, 100%': {
            borderRadius: '60% 40% 30% 70% / 60% 30% 70% 40%',
          },
          '50%': {
            borderRadius: '30% 60% 70% 40% / 50% 60% 30% 60%',
          },
        },
        float: {
          '0%, 100%': {
            transform: 'translateY(0px)',
          },
          '50%': {
            transform: 'translateY(-10px)',
          },
        },
      },
      boxShadow: {
        'soft': '0 4px 20px -2px rgba(0, 0, 0, 0.06)',
        'soft-lg': '0 10px 40px -10px rgba(0, 0, 0, 0.08)',
      },
    },
  },
  plugins: [],
}