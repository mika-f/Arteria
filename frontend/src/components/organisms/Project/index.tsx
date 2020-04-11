import React from "react";
import Select from "react-select";
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
  display: grid;
  grid-template-rows: repeat(2, 32px);
  grid-template-columns: 100px 1fr;
  padding: 8px 0;
`;

const InputStyled = styled(Input)`
  width: 130px;
  height: 24px;
`;

const Label = styled(Small)`
  display: flex;
  align-items: center;
  margin: 0 10px 0 5px;
`;

const Value = styled.div`
  display: flex;
  align-items: center;
`;

const SelectWrapper = styled.div`
  width: 100%;
  color: #333;
`;

const Project: React.FC<Props> = ({ title, executor, executors, onTitleChanged, onExecutorChanged }) => {
  const onSubmitTitle = (event: any) => {
    if (typeof event === "string") {
      if (onTitleChanged) onTitleChanged(event || "untitled");
    }
  };

  const onExecutorSelected = ({ value }: any) => {
    if (onExecutorChanged) onExecutorChanged(executors.find(w => w.tag === value)!);
  };

  const styles = {
    control: (provided: any) => ({
      ...provided,
      borderRadius: 0,
      height: "28px",
      minHeight: "fit-content"
    }),
    valueContainer: (provided: any) => ({
      ...provided,
      maxHeight: "28px"
    }),
    placeholder: (provided: any) => ({
      ...provided,
      fontSize: "12px"
    }),
    singleValue: (provided: any) => ({
      ...provided,
      fontSize: "12px"
    }),
    indicatorsContainer: (provided: any) => ({
      ...provided,
      marginTop: "2px",
      marginBottom: "2px"
    }),
    dropdownIndicator: (provided: any) => ({
      ...provided,
      padding: "2px"
    }),
    menuList: (provided: any) => ({
      ...provided,
      fontSize: "12px"
    })
  };

  return (
    <Container>
      <Label>Project Title : </Label>
      <Value>
        <InputStyled value={title} onSubmit={onSubmitTitle} />
      </Value>
      <Label>Container : </Label>
      <Value>
        <SelectWrapper>
          <Select
            value={{ label: executor.name, value: executor.tag }}
            options={executors.map(w => ({ label: w.name, value: w.tag }))}
            menuPosition="fixed"
            styles={styles}
            onChange={onExecutorSelected}
          />
        </SelectWrapper>
      </Value>
    </Container>
  );
};

export default Project;
