/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ['class'],
  content: [
    './src/**/*.{ts,tsx}'
  ],
  prefix: '',
  theme: {
    container: {
      center: true,
      padding: '2rem',
      screens: {
        '2xl': '1400px'
      }
    },
    extend: {
      fontFamily: {
        poppins: ['Poppins', 'sans-serif']
      },
      colors: {
        border: '#333333',
        input: '#333333',
        ring: '#333333',
        background: '#121212',
        foreground: '#f5faff',
        primary: {
          DEFAULT: '#f5faff',
          foreground: '#ff445a'
        },
        secondary: {
          DEFAULT: '#1c1c1c',
          foreground: '#ff5471'
        },
        destructive: {
          DEFAULT: '#a22f1d',
          foreground: '#f5faff'
        },
        muted: {
          DEFAULT: '#1c1c1c',
          foreground: '#ada5bf'
        },
        accent: {
          DEFAULT: '#1c1c1c',
          foreground: '#f5faff'
        },
        popover: {
          DEFAULT: '#262626',
          foreground: '#f5faff'
        },
        card: {
          DEFAULT: '#1c1c1c',
          foreground: '#f5faff'
        },
      },
      keyframes: {
        'accordion-down': {
          from: { height: '0' },
          to: { height: 'var(--radix-accordion-content-height)' }
        },
        'accordion-up': {
          from: { height: 'var(--radix-accordion-content-height)' },
          to: { height: '0' }
        }
      },
      animation: {
        'accordion-down': 'accordion-down 0.2s ease-out',
        'accordion-up': 'accordion-up 0.2s ease-out'
      },
      transitionProperty: {
        'width': 'width'
      },
    }
  },
  plugins: [require('tailwindcss-animate')],
  corePlugins: {
    preflight: true
  }
}
