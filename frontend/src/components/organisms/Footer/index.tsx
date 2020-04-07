import React from "react";
import styled from "styled-components";

import Container from "../../atoms/Container";
import { Heading1 } from "../../atoms/Headings";
import { Grid, Row } from "../../atoms/FlexboxGrid";
import ListSection from "../ListSection";

const FooterContainer = styled.footer`
  padding: 10px 0 40px 0;
  color: #ccc;
  background-color: #3c3c3c;
`;

const Horizon = styled.div`
  margin: 4px 0 16px 0;
  border-bottom: 1px solid #666;
`;

const Footer: React.FC = () => {
  const items1 = [
    { href: "/about", text: "Arteria について" },
    { href: "/terms", text: "利用規約" },
    { href: "https://github.com/mika-f/Arteria", text: "ソースコード" }
  ];

  const items2 = [
    { href: "/specs", text: "サーバースペック" },
    { href: "/maintenance", text: "メンテナンス情報" },
    { href: "https://www.amazon.jp/hz/wishlist/ls/3TLS0G167KZ28?ref_=wl_share", text: "Amazon 欲しいものリスト" }
  ];

  return (
    <FooterContainer>
      <Container>
        <Heading1>Arteria</Heading1>
        <Grid rows={12}>
          <Row sm={4} md={8}>
            Arteria - Perl5 Playground
          </Row>
          <Row sm={4} md={2}>
            <ListSection title="Arteria" items={items1} />
          </Row>
          <Row sm={4} md={2}>
            <ListSection title="運営情報" items={items2} />
          </Row>
        </Grid>
        <Horizon />
        &copy;
        {` ${new Date().getFullYear()} Fuyuno Mikazuki`}
      </Container>
    </FooterContainer>
  );
};

export default Footer;
