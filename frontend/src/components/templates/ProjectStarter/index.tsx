import React, { useState } from "react";
import Select from "react-select";
import styled from "styled-components";
import { Monaka } from "@mika-f/monaka";

import { PrimaryButton } from "../../atoms/Button";
import Container from "../../atoms/Container";
import Section from "../../molecules/Section";
import Wrapper from "../../organisms/Wrapper";
import { PerlTemplate } from "../../../templates";

type Props = {
  templates: PerlTemplate[];

  onTemplateSelected?: (template: PerlTemplate) => void;
};

const StarterContainer = styled.div`
  width: 100%;
  color: #ccc;
  background-color: #252526;
`;

const SelectWrapper = styled.div`
  margin: 10px 0;
  color: #333;
`;

const EditorContainer = styled.div`
  width: 100%;
  height: 400px;
`;

const ProjectStarter: React.FC<Props> = ({ templates, onTemplateSelected }) => {
  const [currentTemplate, setCurrentTemplate] = useState(templates[0]);

  const onSubmitTemplate = () => {
    if (onTemplateSelected) onTemplateSelected(currentTemplate);
  };

  const onSelectionChanged = ({ value }: any) => {
    setCurrentTemplate(templates.find(w => w.guid === value)!);
  };

  return (
    <Wrapper>
      <StarterContainer>
        <Container>
          <Section size="medium" title="プロジェクトテンプレートを選択してください">
            <SelectWrapper>
              <Select
                value={{ label: currentTemplate.title, value: currentTemplate.guid }}
                options={templates.map(w => ({ label: w.title, value: w.guid }))}
                onChange={onSelectionChanged}
              />
            </SelectWrapper>
            <PrimaryButton onClick={onSubmitTemplate}>作成する！</PrimaryButton>
          </Section>
          <Section size="medium" title="プロジェクトプレビュー (編集しても意味はありません)">
            <EditorContainer>
              <Monaka title="Project Preview" items={currentTemplate.project || []} />
            </EditorContainer>
          </Section>
        </Container>
      </StarterContainer>
    </Wrapper>
  );
};

export default ProjectStarter;
