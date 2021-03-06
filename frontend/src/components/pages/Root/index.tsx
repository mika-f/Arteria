import React, { useEffect, useRef, useState, lazy } from "react";
import { useHistory } from "react-router-dom";
import { SSE } from "sse.js";
import useFetch, { CachePolicies } from "use-http";
import { Item, FileItem, getChildren } from "@mika-f/monaka";

import Loading from "../../templates/Loading";
import ProjectStarterTemplate from "../../templates/ProjectStarter";
import { ProjectInstance, Instance } from "../../../models/instance";
import { Executor } from "../../../models/executor";
import { templates, PerlTemplate } from "../../../templates";
import { Dependency } from "../../../models/dependency";
import { File } from "../../../models/file";

const ProjectEditorTemplate = lazy(() => import(/* webpackChunkName: "editor" */ "../../templates/ProjectEditor"));

const Root: React.FC = () => {
  const [items, setItems] = useState<Item[] | null>(null);
  const [executors, setExecutors] = useState<Executor[]>([]);
  const [instance, setInstance] = useState<ProjectInstance | null>(null);
  const [isReadonly, setIsReadonly] = useState(false);
  // Hmm...
  const [lines, setLines] = useState<string[]>([]);
  const linesBuffer = useRef<string[]>([]);
  const instanceId = useRef<string | null>(null);
  const history = useHistory();

  const [request, response] = useFetch(process.env.ARTERIA_API_SERVER as string, { cachePolicy: CachePolicies.NO_CACHE });

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

    history.push(`/instances/${instanceId.current}`);
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

  const onTitleChanged = (title: string) => {
    if (instance) setInstance({ ...instance, title });
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

  const onItemsChanged = (changesets: Item[]) => {
    const newItems = (items || []).slice();

    for (let i = 0; i < changesets.length; i += 1) {
      const item = changesets[i];
      const index = newItems.findIndex(w => w.id === item.id);
      newItems[index].title = item.title;
      newItems[index].parentId = item.parentId;

      if (item.type === "file") (newItems[index] as FileItem).content = item.content;
    }

    setItems(newItems);
  };

  const onItemCreated = (item: Item) => {
    setItems([...(items || []), item]);
  };

  const onItemDeleted = (item: Item) => {
    let newItems: Item[] = [];
    if (item.type === "file") {
      newItems = (items || []).slice().filter(w => w.id !== item.id);
    } else if (item.type === "directory") {
      newItems = (items || []).slice().filter(w => !getChildren(items || [], item).find(v => v.id === w.id));
    }
    setItems(newItems);
  };

  return (
    <>
      {/* eslint-disable-next-line no-nested-ternary */}
      {executors.length === 0 ? (
        <Loading />
      ) : items === null || instance === null ? (
        <ProjectStarterTemplate templates={templates} onTemplateSelected={onTemplateSelected} />
      ) : (
        <ProjectEditorTemplate
          instance={instance}
          items={items}
          executors={executors}
          lines={lines}
          readonly={isReadonly}
          onTitleChanged={onTitleChanged}
          onDependencyChanged={onDependencyChanged}
          onExecutorChanged={onExecutorSelected}
          onBuildAndPublishClicked={onBuildAndPublishClicked}
          onItemCreated={onItemCreated}
          onItemsChanged={onItemsChanged}
          onItemDeleted={onItemDeleted}
        />
      )}
    </>
  );
};

export default Root;
