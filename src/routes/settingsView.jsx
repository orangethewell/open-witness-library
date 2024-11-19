import React, { useState } from 'react';
import { Box, Button, Divider, FormControl, InputLabel, MenuItem, OutlinedInput, Select, } from '@mui/material';
import { useColorScheme } from '@mui/material/styles';
import { useTranslation } from 'react-i18next';
import { languageList, i18n } from '../i18n';

const SettingsView = () => {
    const { mode, setMode } = useColorScheme();

    const { t, i18n } = useTranslation();
    const [language, setLanguage] = useState(i18n.language);

    const handleChangeLanguage = (language) => {
        i18n.changeLanguage(language)
    }

    return (
        <Box>
            <h1>{t("settings.title")}</h1>
            <Divider/>
            <h2>{t("settings.display")}</h2>
            <FormControl sx={{ m: 1, width: 300 }}>
                <InputLabel id="appearance-label">{t("settings.appearance")}</InputLabel>
                <Select 
                    labelId="appearance-label"
                    id="appearance-select" 
                    value={mode ?? 'system'} 
                    onChange={(event) =>
                        setMode(event.target.value)
                      }
                    input={<OutlinedInput label={t("settings.appearance")} />}
                    MenuProps={{
                        PaperProps: {
                          style: {
                            maxHeight: 48 * 4.5 + 8,
                            width: 250,
                          },
                        },
                    }}
                >
                    <MenuItem value="system">{t("settings.appearance_selectors.default")}</MenuItem>
                    <MenuItem value="light">{t("settings.appearance_selectors.light")}</MenuItem>
                    <MenuItem value="dark">{t("settings.appearance_selectors.dark")}</MenuItem>
                </Select>
            </FormControl>
            <p>
            <FormControl sx={{ m: 1, width: 300 }}>
                <InputLabel id="language-label">{t("settings.language")}</InputLabel>
                <Select 
                    labelId="language-label"
                    id="language-select" 
                    value={language} 
                    onChange={(ev) => {
                        setLanguage(ev.target.value);
                        handleChangeLanguage(ev.target.value)
                    }}
                    input={<OutlinedInput label={t("settings.language")} />}
                    MenuProps={{
                        PaperProps: {
                          style: {
                            maxHeight: 48 * 4.5 + 8,
                            width: 120,
                          },
                        },
                    }}
                >
                    {languageList.map((lang) => (
                        <MenuItem key={lang.code} value={lang.code}><p style={{margin: 0}}>{t(`settings.language_selectors.${lang.code}`)}<br/><span className='subtext'>{lang.native}</span></p></MenuItem>
                    ))}
                </Select>
            </FormControl>
            </p>
        </Box>
    );
};

export default SettingsView;
