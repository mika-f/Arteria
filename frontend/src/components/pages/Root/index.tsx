import React, { useEffect, useState } from "react";
import useFetch from "use-http";
import { Item } from "@mika-f/monaka";

import ProjectEditorTemplate from "../../templates/ProjectEditor";
import ProjectStarterTemplate from "../../templates/ProjectStarter";
import { ProjectInstance } from "../../../models/instance";
import { Executor } from "../../../models/executor";
import { templates, PerlTemplate } from "../../../templates";
import { Dependency } from "../../../models/dependency";

const Root: React.FC = () => {
  const [items, setItems] = useState<Item[] | null>(null);
  const [executors, setExecutors] = useState<Executor[]>([]);
  const [instance, setInstance] = useState<ProjectInstance | null>(null);
  const [request, response] = useFetch(process.env.ARTERIA_API_SERVER as string);

  useEffect(() => {
    // eslint-disable-next-line no-use-before-define
    fetchExecutors();
  }, []);

  const fetchExecutors = async () => {
    const res = await request.get("/executors");
    if (response.ok) setExecutors(res);
  };

  const onTemplateSelected = (template: PerlTemplate) => {
    setInstance({
      title: "untitled",
      executor: executors[0],
      dependencies: template.dependencies,
      files: []
    } as ProjectInstance);
    setItems(template.project);
  };

  const onDependencyChanged = (dependencies: Dependency[]) => {
    if (instance) setInstance({ ...instance, dependencies });
  };

  const onExecutorSelected = (executor: Executor) => {
    if (instance) setInstance({ ...instance, executor });
  };

  console.log(instance);

  return (
    <>
      {items === null || instance === null ? (
        <ProjectStarterTemplate templates={templates} onTemplateSelected={onTemplateSelected} />
      ) : (
        <ProjectEditorTemplate instance={instance} items={items} executors={executors} onDependencyChanged={onDependencyChanged} onExecutorChanged={onExecutorSelected} />
      )}
    </>
  );
};

export default Root;
