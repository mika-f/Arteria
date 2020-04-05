import React from "react";
import styled from "styled-components";

import Console from ".";

const LimitedWidth = styled.div`
  width: 750px;
`;

const console = `\
stdout: mikazuki@Lydie MINGW64 /f/repos/github.com/mika-f/Arteria/frontend (develop)
stdout: $ yarn run storybook
stdout: yarn run v1.10.1
stdout: $ start-storybook -p 6006
stdout: info @storybook/react v5.3.18
stdout: info
stdout: info => Loading presets
stdout: info => Loading presets
stdout: info => Loading custom manager config.
stdout: info => Adding stories defined in ".storybook/main.js".
stdout: info => Using default Webpack setup.
stdout: info => Using base config because react-scripts is not installed.
stdout: webpack built ce8b8dfa37fadd286cb0 in 25252ms
stdout: ╭─────────────────────────────────────────────────╮
stdout: │                                                 │
stdout: │   Storybook 5.3.18 started                      │
stdout: │   30 s for manager and 29 s for preview         │
stdout: │                                                 │
stdout: │    Local:            http://localhost:6006/     │
stdout: │    On your network:  http://192.0.2.43:6006/    │
stdout: │                                                 │
stdout: ╰─────────────────────────────────────────────────╯
stdout: 
stderr: × ｢wdm｣: Hash: ff1558f79f730e74a143
stderr: Version: webpack 4.42.1
stderr: Time: 1533ms
stderr: Built at: 2020-04-05 17:39:29
stdout: webpack building...
stdout: webpack built 3ddde670c804478bbad4 in 1454ms\
`.split("\n");

export default {
  title: "atoms/Console"
};

export const Default = () => (
  <LimitedWidth>
    <Console lines={console} />
  </LimitedWidth>
);
