import styled from "styled-components";

const BaseParagraph = styled.p`
  margin: 0;
`;

const XSmall = styled(BaseParagraph)`
  font-size: 12px;
`;

const Small = styled(BaseParagraph)`
  font-size: 14px;
`;

const Normal = styled(BaseParagraph)`
  font-size: 16px;
`;

const Large = styled(BaseParagraph)`
  font-size: 18px;
`;

const XLarge = styled(BaseParagraph)`
  font-size: 21px;
`;

export { BaseParagraph as Paragraph, XSmall, Small, Normal, Large, XLarge };
