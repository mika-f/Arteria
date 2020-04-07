import React from "react";
import styled from "styled-components";

import Header from "../Header";
import Footer from "../Footer";

const Container = styled.div`
  display: flex;
  flex-flow: column nowrap;
  height: 100%;
  min-height: 100vh;
`;

const Content = styled.div`
  flex: 1 1 auto;
`;

const Wrapper: React.FC = ({ children }) => {
  return (
    <Container>
      <Header />
      <Content>{children}</Content>
      <Footer />
    </Container>
  );
};

export default Wrapper;
