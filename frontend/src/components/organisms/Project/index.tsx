import React from "react";
import Select from "react-select";
import styled from "styled-components";
import { Input } from "@mika-f/monaka";

import { PrimaryButton } from "../../atoms/Button";
import { Small } from "../../atoms/Typography";
import { Executor } from "../../../models/executor";

type Props = {
  title: string;
  executor: Executor;
  executors: Executor[];
  readonly: boolean;

  onTitleChanged?: (title: string) => void;
  onExecutorChanged?: (executor: Executor) => void;
  onClickBuild?: () => void;
};

const Container = styled.div`
  display: grid;
  grid-template-rows: repeat(3, 32px);
  grid-template-columns: 120px 1fr;
  padding: 8px 0;
`;

const InputStyled = styled(Input)`
  width: 110px;
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

const LabelAndValue = styled.div`
  grid-column: 1/3;
`;

const Button = styled(PrimaryButton)`
  width: 100%;
  height: 30px;
  padding: 4px;
  font-size: 14px;
`;

const SelectWrapper = styled.div`
  width: 100%;
  color: #333;
`;

const Project: React.FC<Props> = ({ title, executor, executors, readonly, onTitleChanged, onExecutorChanged, onClickBuild }) => {
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
        <InputStyled value={title} mode="PropertyChanged" disabled={readonly} onSubmit={onSubmitTitle} />
      </Value>
      <Label>Container : </Label>
      <Value>
        <SelectWrapper>
          <Select
            value={{ label: executor.name, value: executor.tag }}
            options={executors.map(w => ({ label: w.name, value: w.tag }))}
            menuPosition="fixed"
            styles={styles}
            isDisabled={readonly}
            onChange={onExecutorSelected}
          />
        </SelectWrapper>
      </Value>
      <LabelAndValue>
        <Button disabled={readonly} onClick={onClickBuild}>
          Build &amp; Publish
        </Button>
      </LabelAndValue>
    </Container>
  );
};

export default Project;
