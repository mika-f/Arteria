import React, { useState } from "react";
import { Transition } from "react-transition-group";
import { TransitionStatus } from "react-transition-group/Transition";
import styled from "styled-components";

import { LinkButton } from "../../atoms/Button";
import FontAwesome from "../../atoms/FontAwesome";
import Hyperlink from "../../atoms/HyperLink";

type Props = {
  name: string;
  version: string | null;
  editable: boolean;

  onClickDelete?: (nameWithVersion: string) => void;
};

const FlexboxContainer = styled.div`
  display: flex;
  margin: 2px 0;
  overflow: hidden;
`;

const FadeAnimation = styled.div<{ state: TransitionStatus }>`
  display: inline-block;
  width: ${({ state }) => (state === "entering" || state === "exited" ? "0" : "24px")};
  transition: 0.1s;
  transform: ${({ state }) => (state === "entering" || state === "exited" ? "translateX(20px)" : "translateX(0)")};
`;

const Name = styled.div`
  flex: 0 1 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

const Version = styled.div`
  display: inline-block;
  flex: 0 0 auto;
  margin: 0 5px;
  color: #999;
  text-overflow: ellipsis;
`;

const DependencyItem: React.FC<Props> = ({ name, version, editable, onClickDelete }) => {
  const [hover, setHover] = useState(false);

  const onMouseEnter = () => setHover(true);
  const onMouseLeave = () => setHover(false);
  const OnClickButton = () => {
    if (onClickDelete) {
      if (version) onClickDelete(`${name}@${version}`);
      else onClickDelete(name);
    }
  };

  return (
    <FlexboxContainer onMouseEnter={onMouseEnter} onMouseLeave={onMouseLeave}>
      <Name>
        <Hyperlink href={`https://metacpan.org/pod/${name}`} target="_blank" rel="noreferrer noopener">
          {name}
        </Hyperlink>
      </Name>
      <Version>
        {version || "(null)"}
        <Transition in={editable && hover} timeout={100}>
          {state => (
            <FadeAnimation state={state}>
              <LinkButton onClick={OnClickButton}>
                <FontAwesome icon="trash" prefix="regular" fixed />
              </LinkButton>
            </FadeAnimation>
          )}
        </Transition>
      </Version>
    </FlexboxContainer>
  );
};

export default DependencyItem;
