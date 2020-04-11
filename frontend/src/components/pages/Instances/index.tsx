import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import useFetch from "use-http";
import { v4 as uuid } from "uuid";
import { DirectoryItem, FileItem, Item } from "@mika-f/monaka";

import Loading from "../../templates/Loading";
import ProjectReaderTemplate from "../../templates/ProjectReader";
import { File } from "../../../models/file";
import { ProjectInstance } from "../../../models/instance";

const Instances: React.FC = () => {
  const { instanceId } = useParams();
  const [items, setItems] = useState<Item[] | null>(null);
  const [instance, setInstance] = useState<ProjectInstance | null>(null);
  const [lines, setLines] = useState<string[]>([]);

  const [request, response] = useFetch(process.env.ARTERIA_API_SERVER as string);

  useEffect(() => {
    // eslint-disable-next-line no-use-before-define
    fetchExecutors();
  }, []);

  const fetchExecutors = async () => {
    if (!instanceId) return;

    const res = await request.get(`/instances/${instanceId}`);
    if (response.ok) {
      const filesToItems = (files: File[]): Item[] => {
        // eslint-disable-next-line no-shadow
        const items: Item[] = [];

        for (let i = 0; i < files.length; i += 1) {
          const file = files[i];
          const paths = file.title.split("/");
          let parentId: string | null = null;

          for (let j = 0; j < paths.length; j += 1) {
            if (j === paths.length - 1) {
              // filename
              items.push({ type: "file", id: uuid(), title: paths[j], content: file.content, parentId } as FileItem);
            } else {
              // dirname
              // eslint-disable-next-line no-loop-func
              const alreadyHasItem = items.find(w => w.title === paths[j] && w.id === parentId);

              if (alreadyHasItem) {
                parentId = alreadyHasItem.id;
              } else {
                const id = uuid();
                items.push({ type: "directory", id, title: paths[j], parentId, state: "closed" } as DirectoryItem);
                parentId = id;
              }
            }
          }
        }

        return items;
      };

      setInstance({ title: res.title, executor: { name: res.executor }, dependencies: res.dependencies } as ProjectInstance);
      setLines(res.result.split("\n"));
      setItems(filesToItems(res.files));
    }
  };

  return <>{items === null || instance === null ? <Loading /> : <ProjectReaderTemplate instance={instance} items={items} lines={lines} />}</>;
};

export default Instances;
