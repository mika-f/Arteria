import React from "react";
import styled from "styled-components";

import Container from ".";

const ColoredContainer = styled(Container)`
  background-color: gray;
`;

export default {
  title: "atoms/Container"
};

export const Default = () => <ColoredContainer>Hello, Storybook</ColoredContainer>;
