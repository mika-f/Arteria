import React, { useEffect, useRef, useState } from "react";
import { SSE } from "sse.js";
import useFetch from "use-http";
import { Item, FileItem } from "@mika-f/monaka";

import ProjectEditorTemplate from "../../templates/ProjectEditor";
import ProjectStarterTemplate from "../../templates/ProjectStarter";
import { ProjectInstance, Instance } from "../../../models/instance";
import { Executor } from "../../../models/executor";
import { templates, PerlTemplate } from "../../../templates";
import { Dependency } from "../../../models/dependency";
import { File } from "../../../models/file";

const Root: React.FC = () => {
  const [items, setItems] = useState<Item[] | null>(null);
  const [executors, setExecutors] = useState<Executor[]>([]);
  const [instance, setInstance] = useState<ProjectInstance | null>(null);
  const [isReadonly, setIsReadonly] = useState(false);
  // Hmm...
  const [lines, setLines] = useState<string[]>([]);
  const linesBuffer = useRef<string[]>([]);
  const instanceId = useRef<string | null>(null);

  const [request, response] = useFetch(process.env.ARTERIA_API_SERVER as string);

  useEffect(() => {
    // eslint-disable-next-line no-use-before-define
    fetchExecutors();
  }, []);

  const fetchExecutors = async () => {
    const res = await request.get("/executors");
    if (response.ok) setExecutors(res);
  };

  const insertNewLine = (line: string) => {
    linesBuffer.current = [...linesBuffer.current, line];
    setLines(linesBuffer.current);
  };

  const redirect = () => {
    if (!instance || !instanceId.current) return;

    history.pushState(null, instance?.title, `/instances/${instanceId.current}`);
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

  const onBuildAndPublishClicked = () => {
    if (!instance) return;

    setIsReadonly(true);

    const itemsToFiles = (targets: Item[]): File[] => {
      const files: File[] = [];

      const filtered: Item[] = targets.filter(w => w.type === "file");
      for (let i = 0; i < filtered.length; i += 1) {
        const file = filtered[i] as FileItem;
        const paths: string[] = [];

        let current: Item | undefined = file;

        do {
          paths.push(current?.title);
          // eslint-disable-next-line no-loop-func
          current = targets.find(w => w.id === current?.parentId);
        } while (current);

        files.push({
          title: paths.reverse().join("/"),
          content: file.content
        } as File);
      }

      return files;
    };

    const payload = {
      title: instance.title,
      executor: instance.executor.tag,
      dependencies: instance.dependencies,
      files: itemsToFiles(items || [])
    } as Instance;

    const eventsource = new SSE(`${process.env.ARTERIA_API_SERVER}/instances`, { headers: { "Content-Type": "application/json" }, payload: JSON.stringify(payload) } as any);
    eventsource.addEventListener("message", e => {
      const { event, data } = JSON.parse(e.data);

      switch (event) {
        case "Message":
          insertNewLine(data);
          break;

        case "Heartbeat":
          break;

        case "Command":
          // eslint-disable-next-line no-case-declarations
          const { command, value } = data;

          switch (command) {
            case "close":
              eventsource.close();
              redirect();
              break;

            case "redirect":
              instanceId.current = value;
              break;

            default:
              break;
          }
          break;

        default:
          break;
      }
    });

    (eventsource as any).stream();
  };

  return (
    <>
      {items === null || instance === null ? (
        <ProjectStarterTemplate templates={templates} onTemplateSelected={onTemplateSelected} />
      ) : (
        <ProjectEditorTemplate
          instance={instance}
          items={items}
          executors={executors}
          lines={lines}
          readonly={isReadonly}
          onDependencyChanged={onDependencyChanged}
          onExecutorChanged={onExecutorSelected}
          onBuildAndPublishClicked={onBuildAndPublishClicked}
        />
      )}
    </>
  );
};

export default Root;
