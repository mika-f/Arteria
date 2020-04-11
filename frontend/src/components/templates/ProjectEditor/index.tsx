import React from "react";
import styled from "styled-components";
import { Monaka, Item, ProjectSection } from "@mika-f/monaka";

import Console from "../../atoms/Console";
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
  lines: string[];
  readonly: boolean;

  // template events
  onTitleChanged?: (title: string) => void;
  onDependencyChanged?: (dependencies: Dependency[]) => void;
  onExecutorChanged?: (executor: Executor) => void;
  onBuildAndPublishClicked?: () => void;

  // editor events
  onItemCreated?: (item: Item) => void;
  onItemsChanged?: (items: Item[]) => void;
  onItemDeleted?: (item: Item) => void;
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

const ProjectEditorTemplate: React.FC<Props> = ({
  instance,
  items,
  executors,
  lines,
  readonly,
  onTitleChanged,
  onDependencyChanged,
  onExecutorChanged,
  onBuildAndPublishClicked,
  onItemCreated,
  onItemsChanged,
  onItemDeleted
}) => {
  return (
    <Wrapper>
      <Container>
        <MonakaContainer>
          <Monaka title="Arteria Project" items={items} readonly={readonly} onItemCreated={onItemCreated} onItemsChanged={onItemsChanged} onItemDeleted={onItemDeleted}>
            <ProjectSection title="Project">
              <Project
                title={instance.title}
                executor={instance.executor}
                executors={executors}
                readonly={readonly}
                onTitleChanged={onTitleChanged}
                onExecutorChanged={onExecutorChanged}
                onClickBuild={onBuildAndPublishClicked}
              />
            </ProjectSection>
            <ProjectSection title="Dependencies">
              <Dependencies dependencies={instance.dependencies} editable={!readonly} onDependenciesChanged={onDependencyChanged} />
            </ProjectSection>
          </Monaka>
        </MonakaContainer>
        {lines.length > 0 ? <Console lines={lines} /> : null}
      </Container>
    </Wrapper>
  );
};

export default ProjectEditorTemplate;
