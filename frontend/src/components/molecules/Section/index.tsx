import React from "react";
import styled from "styled-components";

import { Heading2 } from "../../atoms/Headings";
import { Normal } from "../../atoms/Typography";

type Props = {
  title: string;
};

const Title = styled(Heading2)`
  margin: 0 0 0.25em 0;
`;

const InnerSection = styled.section`
  margin: 0.25em 0 1em 0;
`;

const Section: React.FC<Props> = ({ title, children }) => {
  return (
    <InnerSection>
      <Title>{title}</Title>
      <Normal>{children}</Normal>
    </InnerSection>
  );
};

export default Section;
