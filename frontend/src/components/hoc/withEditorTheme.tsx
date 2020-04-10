import React from "react";
import { LanguageProvider, IconProvider, FileIcon, FileType, ThemeProvider } from "@mika-f/monaka";
import { Theme } from "@mika-f/monaka/dist/components/ThemeProvider";

const icons = [] as FileIcon[];

const languages = [
  {
    extension: /\.pl$/,
    language: "perl"
  },
  {
    extension: /\.pm$/,
    language: "perl"
  },
  {
    extension: /\.t$/,
    language: "perl"
  }
] as FileType[];

const theme = {
  base: "vs-dark",
  activeBackground: "#1e1e1e",
  editorBackground: "#3c3c3c",
  errorBackground: "#94124e",
  inactiveBackground: "#2d2d2d",
  highlightBackground: "#094771",
  hoverBackground: "#2a2d2e",
  background: "#252526",
  activeBorderColor: "#094771",
  errorBorderColor: "#ff3d71",
  fontColor: "#ccc",
  activeFontColor: "#fff",
  errorFontColor: "#fff"
} as Theme;

const withEditorTheme = (WrappedComponent: React.ComponentType<any>): React.FC => {
  const Hoc: React.FC<any> = props => {
    return (
      <LanguageProvider languages={languages}>
        <IconProvider icons={icons}>
          <ThemeProvider theme={theme}>
            {/* eslint-disable-next-line react/jsx-props-no-spreading */}
            <WrappedComponent {...props} />
          </ThemeProvider>
        </IconProvider>
      </LanguageProvider>
    );
  };

  return Hoc;
};

export default withEditorTheme;
