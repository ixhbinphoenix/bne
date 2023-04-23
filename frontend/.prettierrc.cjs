module.exports = {
  plugins: [require.resolve("prettier-plugin-astro")],
  overrides: [
    {
      files: "*.astro",
      options: {
        parser: "astro"
      }
    }
  ],
  tabWidth: 2,
  semi: true,
  bracketSameLine: true,
  astroAllowShorthand: true,
  arrowParens: "always",
  printWidth: 120,
  endOfLine: "lf",
  trailingComma: "none"
};
