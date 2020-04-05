import React from "react";
import { storiesOf } from "@storybook/react";

import { XSmall, Small, Normal, Large, XLarge } from ".";

const Lorem = `Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.`;

storiesOf("atoms/Typographies", module).add("default", () => (
  <>
    <XSmall>{Lorem}</XSmall>
    <Small>{Lorem}</Small>
    <Normal>{Lorem}</Normal>
    <Large>{Lorem}</Large>
    <XLarge>{Lorem}</XLarge>
  </>
));
