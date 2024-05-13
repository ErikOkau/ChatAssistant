// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },

  ssr: false,
  srcDir: "src/",
  routeRules: {
    "/": { prerender: true },
  },
  modules: ["nuxt-quasar-ui"],
  quasar: {
    iconSet: "material-symbols-rounded",
    extras: {
      fontIcons: [
        "material-symbols-rounded",
        "material-symbols-outlined",
        "material-symbols-sharp",
        "material-icons-outlined",
        "material-icons-round",
        "material-icons-sharp",
        "material-icons",
      ],

      svgIcons: [
        "material-symbols-rounded",
        "material-symbols-outlined",
        "material-symbols-sharp",
        "material-icons-outlined",
        "material-icons-round",
        "material-icons-sharp",
        "material-icons",
      ],
    },

    sassVariables: "src/assets/scss/quasar.variables.scss",
    config: {
      brand: {
        primary: "#000000",
        secondary: "#d9d9d9",
        accent: "#838383",
        dark: "#727272",

        positive: "#21BA45",
        negative: "#C10015",
        info: "#31CCEC",
        warning: "#F2C037",
      },
    },
  },
})
