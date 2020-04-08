import React from "react";
import styled from "styled-components";

import Container from "../../atoms/Container";
import { Normal } from "../../atoms/Typography";
import Wrapper from "../../organisms/Wrapper";
import Section from "../../molecules/Section";

const AboutContainer = styled(Container)`
  margin: 40px auto 0;
`;

const AboutTemplate: React.FC = () => {
  return (
    <Wrapper>
      <AboutContainer>
        <Section size="large" title="Arteria について">
          <Normal>Arteria とは、以下の特徴を備えたオンライン Perl5 Playground です。</Normal>
          <ul>
            <li>誰でも無料で使うこと出来ます。</li>
            <li>アカウント登録は必要ありません。</li>
            <li>Visual Studio Code (Monaco Editor) でコードが記述できます。</li>
            <li>CPAN モジュールを使うことが可能です。</li>
            <li>自分自身で Arteria をホストすることが可能です。</li>
            <li>ソースコードは公開されています。</li>
          </ul>
          <Normal>Arteria は短期間運営されていた Altar の後継バージョンです。以下の点が異なります。</Normal>
          <ul>
            <li>より自身の環境を作りやすくなりました。</li>
            <li>より早く処理結果が返されるようになりました。</li>
          </ul>
        </Section>
      </AboutContainer>
    </Wrapper>
  );
};

export default AboutTemplate;
