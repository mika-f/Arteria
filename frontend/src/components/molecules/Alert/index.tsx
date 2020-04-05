import React from "react";
import styled from "styled-components";

import { Heading2 } from "../../atoms/Headings";
import { Normal } from "../../atoms/Typography";

type Variant = "info" | "warning" | "error";

type Props = {
  title: string;
  variant: Variant;
};

const getVariantColor = (variant: Variant, isBackground: boolean): string => {
  switch (variant) {
    case "info":
      return isBackground ? "#c7e2ff" : "#0095ff";

    case "warning":
      return isBackground ? "#fff1c2" : "#ffaa00";

    case "error":
      return isBackground ? "#ffd6d9" : "#ff3d71";

    default:
      return "white";
  }
};

const Container = styled.div<{ variant: Variant }>`
  border-top: 8px solid ${props => getVariantColor(props.variant, false)};
  background: ${props => getVariantColor(props.variant, true)};
  color: ${props => getVariantColor(props.variant, false)};
  padding: 4px 0px 4px 8px;
`;

const Title = styled(Heading2)`
  margin: 0 0 0.25em 0;
`;

const Content = styled(Normal)`
  color: #333;
`;

const Alert: React.FC<Props> = ({ title, variant, children }) => {
  return (
    <Container variant={variant}>
      <Title>{title}</Title>
      <Content>{children}</Content>
    </Container>
  );
};

export default Alert;
