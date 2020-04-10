import React from "react";
import styled from "styled-components";
import { Input } from "@mika-f/monaka";

import { Small } from "../../atoms/Typography";
import { Executor } from "../../../models/executor";

type Props = {
  title: string;
  executor: Executor;
  executors: Executor[];

  onTitleChanged?: (title: string) => void;
  onExecutorChanged?: (executor: Executor) => void;
};

const Container = styled.div`
  display: flex;
  padding: 8px 0;
`;

const Project: React.FC<Props> = ({ title, executor, executors, onTitleChanged, onExecutorChanged }) => {
  const onSubmitTitle = (event: any) => {
    if (typeof event === "string") {
      if (onTitleChanged) onTitleChanged(event || "untitled");
    }
  };

  return (
    <Container>
      <Small>Project Title : </Small>
      <Input value={title} onSubmit={onSubmitTitle} />
    </Container>
  );
};

export default Project;
