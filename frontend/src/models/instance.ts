import { Dependency } from "./dependency";
import { Executor } from "./executor";
import { File } from "./file";

export type ProjectInstance = {
  title: string;
  executor: Executor;
  dependencies: Dependency[];
  files: File[];
};

export type Instance = {
  title: string;
  executor: string;
  status: string;
  result: string;
  dependencies: Dependency[];
  files: File[];
};
