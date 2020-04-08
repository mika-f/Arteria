import React from "react";
import styled from "styled-components";

import Container from "../../atoms/Container";
import { Normal } from "../../atoms/Typography";
import Wrapper from "../../organisms/Wrapper";
import Section from "../../molecules/Section";

type Props = {
  hasGoogleAnalyticsTag: boolean;
};

const PrivacyContainer = styled(Container)`
  margin: 40px auto 0;
`;

const PrivacyTemplate: React.FC<Props> = ({ hasGoogleAnalyticsTag }) => {
  return (
    <Wrapper>
      <PrivacyContainer>
        <Section size="large" title="プライバシーポリシー">
          当サイトでは、個人情報の取り扱いについて、以下のようにプライバシーポリシーを定めています。
        </Section>

        {hasGoogleAnalyticsTag ? (
          <Section size="medium" title="Google Analytics">
            <Normal>
              当サイトでは、Googleによるアクセス解析ツール「Googleアナリティクス」を利用しています。
              このGoogleアナリティクスはトラフィックデータの収集のためにCookieを使用しています。 このトラフィックデータは匿名で収集されており、個人を特定するものではありません。
              この機能はCookieを無効にすることで収集を拒否することが出来ますので、お使いのブラウザの設定をご確認ください。
            </Normal>
          </Section>
        ) : null}

        <Section size="medium" title="免責事項">
          <Normal>
            当サイトからリンクやバナーなどによって他のサイトに移動された場合、移動先サイトで提供される情報、サービス等について一切の責任を負いません。
            また、当サイトに掲載された内容によって生じた損害等の一切の責任を負いかねますのでご了承ください。
          </Normal>
        </Section>

        <Section size="medium" title="プライバシーポリシーの変更について">
          当サイトは、個人情報に関して適用される日本の法令を遵守するとともに、本ポリシーの内容を適宜見直しその改善に努めます。
          修正された最新のプライバシーポリシーは常に本ページにて開示されます。
        </Section>
      </PrivacyContainer>
    </Wrapper>
  );
};

export default PrivacyTemplate;
