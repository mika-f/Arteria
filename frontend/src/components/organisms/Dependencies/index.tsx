import React from "react";
import styled from "styled-components";
import { Input } from "@mika-f/monaka";

import { Normal } from "../../atoms/Typography";
import DependencyItem from "../../molecules/DependencyItem";
import { Dependency } from "../../../models/dependency";

type Props = {
  dependencies: Dependency[];
  editable: boolean;
  onDependenciesChanged?: (dependencies: Dependency[]) => void;
};

const Container = styled.div`
  margin: 0 10px;
`;

const InputStyled = styled(Input)`
  width: calc(100% - 10px);
  height: 24px;
`;

const Dependencies: React.FC<Props> = ({ dependencies, editable, onDependenciesChanged }) => {
  const onDependencyDeleted = (nameWithVersion: string) => {
    const deps = dependencies.filter(w => w.nameWithVersion !== nameWithVersion);
    if (onDependenciesChanged) onDependenciesChanged(deps);
  };

  const onDependencyAdded = (nameWithVersion: any) => {
    if (typeof nameWithVersion === "string") if (onDependenciesChanged) onDependenciesChanged([...dependencies, { nameWithVersion }]);
  };

  return (
    <Container>
      {dependencies.length === 0 && !editable ? <Normal>No Dependencies</Normal> : null}
      {dependencies.map(w => {
        const [name, version] = w.nameWithVersion.split("@");

        return <DependencyItem key={w.nameWithVersion} name={name} version={version} editable={editable} onClickDelete={onDependencyDeleted} />;
      })}
      {editable ? <InputStyled value="" mode="Submit" placeholder="Data::Validator@1.07" onSubmit={onDependencyAdded} /> : null}
    </Container>
  );
};

export default Dependencies;
