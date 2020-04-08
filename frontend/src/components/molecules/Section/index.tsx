import React from "react";
import styled from "styled-components";

import { Heading1, Heading2, Heading3 } from "../../atoms/Headings";
import { Normal } from "../../atoms/Typography";

type Props = {
  size: "small" | "medium" | "large";
  title: string;
};

const TitleH1 = styled(Heading1)`
  margin: 0 0 0.25em 0;
`;

const TitleH2 = styled(Heading2)`
  margin: 0 0 0.5em 0;
`;

const TitleH3 = styled(Heading3)`
  margin: 0 0 0.75em 0;
`;

const InnerSection = styled.section`
  margin: 0.5em 0 2em 0;
`;

const Section: React.FC<Props> = ({ size, title, children }) => {
  // eslint-disable-next-line no-shadow
  const getComponentForSize = (size: Props["size"], title: string) => {
    switch (size) {
      case "large":
        return <TitleH1>{title}</TitleH1>;

      case "medium":
        return <TitleH2>{title}</TitleH2>;

      case "small":
        return <TitleH3>{title}</TitleH3>;

      default:
        return null;
    }
  };
  return (
    <InnerSection>
      {getComponentForSize(size, title)}
      {children}
    </InnerSection>
  );
};

export default Section;
