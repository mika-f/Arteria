import React from "react";
import styled from "styled-components";
import { Monaka, Item, ProjectSection } from "@mika-f/monaka";

import Dependencies from "../../organisms/Dependencies";
import Project from "../../organisms/Project";
import Wrapper from "../../organisms/Wrapper";
import { Executor } from "../../../models/executor";
import { ProjectInstance } from "../../../models/instance";
import { Dependency } from "../../../models/dependency";

type Props = {
  instance: ProjectInstance;
  items: Item[];
  executors: Executor[];

  // template events
  onTitleChanged?: (title: string) => void;
  onDependencyChanged?: (dependencies: Dependency[]) => void;
  onExecutorChanged?: (executor: Executor) => void;

  // editor events
  onItemCreated?: (item: Item) => void;
  onItemsChanged?: (items: Item[]) => void;
  onItemDeleted?: (item: Item) => void;
};

const Container = styled.div`
  width: 100%;
  min-height: 500px;
  color: #ccc;
`;

const ProjectEditorTemplate: React.FC<Props> = ({
  instance,
  items,
  executors,
  onTitleChanged,
  onDependencyChanged,
  onExecutorChanged,
  onItemCreated,
  onItemsChanged,
  onItemDeleted
}) => {
  return (
    <Wrapper>
      <Container>
        <Monaka title="Arteria Project" items={items} onItemCreated={onItemCreated} onItemsChanged={onItemsChanged} onItemDeleted={onItemDeleted}>
          <ProjectSection title="Project">
            <Project title={instance.title} executor={instance.executor} executors={executors} onTitleChanged={onTitleChanged} onExecutorChanged={onExecutorChanged} />
          </ProjectSection>
          <ProjectSection title="Dependencies">
            <Dependencies dependencies={instance.dependencies} editable onDependenciesChanged={onDependencyChanged} />
          </ProjectSection>
        </Monaka>
      </Container>
    </Wrapper>
  );
};

export default ProjectEditorTemplate;
