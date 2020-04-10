import { Item } from "@mika-f/monaka";

import * as DataDumper from "./DataDumper";
import * as DataValidator from "./DataValidator";
import * as HelloWorld from "./HelloWorld";
import { Dependency } from "../models/dependency";

export type PerlTemplate = {
  guid: string;
  title: string;
  dependencies: Dependency[];
  command: string;
  project: Item[];
};

const templates: PerlTemplate[] = [DataDumper, DataValidator, HelloWorld];

export { templates };
