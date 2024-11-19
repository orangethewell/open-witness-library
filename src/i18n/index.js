import i18n from 'i18next';
import I18nextBrowserLanguageDetector from "i18next-browser-languagedetector";
import { initReactI18next } from "react-i18next";
import translations from "./locales"

const i18nConfig = {
  resources: translations,
  fallbackLng: 'en-US',
  defaultNS: 'translations'
}

const languageList = [
  {code: "en-US", native: "English"},
  {code: "pt-BR", native: "Português (Brasil)"},
]

i18n
  .use(I18nextBrowserLanguageDetector) // Usa o detector de idioma do seu browser
  .use(initReactI18next) // Usa o pacote do i18n específico para React
  .init(i18nConfig) // Usa nossas configurações

export { i18n, languageList}