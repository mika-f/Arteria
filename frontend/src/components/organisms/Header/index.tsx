import React from "react";
import styled from "styled-components";

import Container from "../../atoms/Container";
import { Heading2 } from "../../atoms/Headings";
import RouterLink from "../../atoms/RouterLink";

const HeaderContainer = styled.header`
  padding: 10px 0;
  color: #ccc;
  background-color: #3c3c3c;
`;

const Header: React.FC = () => {
  return (
    <HeaderContainer>
      <Container>
        <RouterLink to="/">
          <Heading2>Arteria</Heading2>
        </RouterLink>
      </Container>
    </HeaderContainer>
  );
};

export default Header;
