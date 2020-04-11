import React from "react";
import styled from "styled-components";
import { Monaka, Item, ProjectSection } from "@mika-f/monaka";

import Console from "../../atoms/Console";
import Dependencies from "../../organisms/Dependencies";
import Project from "../../organisms/Project";
import Wrapper from "../../organisms/Wrapper";
import { ProjectInstance } from "../../../models/instance";

type Props = {
  instance: ProjectInstance;
  items: Item[];
  lines: string[];
};

const Container = styled.div`
  width: 100%;
  color: #ccc;
  background: #252526;
`;

const MonakaContainer = styled.div`
  height: 500px;
  min-height: 500px;
`;

const ProjectReaderTemplate: React.FC<Props> = ({ instance, items, lines }) => {
  return (
    <Wrapper>
      <Container>
        <MonakaContainer>
          <Monaka title="Arteria Project" items={items} readonly>
            <ProjectSection title="Project">
              <Project title={instance.title} executor={instance.executor} executors={[]} readonly />
            </ProjectSection>
            <ProjectSection title="Dependencies">
              <Dependencies dependencies={instance.dependencies} editable={false} />
            </ProjectSection>
          </Monaka>
        </MonakaContainer>
        {lines.length > 0 ? <Console lines={lines} /> : null}
      </Container>
    </Wrapper>
  );
};

export default ProjectReaderTemplate;
