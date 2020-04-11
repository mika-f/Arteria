import React from "react";
import styled from "styled-components";

import { Normal } from "../../atoms/Typography";
import Wrapper from "../../organisms/Wrapper";

const Centering = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  color: #ccc;
  background: #252526;
`;

const Loading: React.FC = () => {
  return (
    <Wrapper>
      <Centering>
        <Normal>読み込み中...</Normal>
      </Centering>
    </Wrapper>
  );
};

export default Loading;
